"""
This module handles the user interface logic for Anya Wallet using Kivy, 
with integration of Gemini AI for financial insights.
"""

import time

from kivy.app import App
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.uix.popup import Popup
from kivy.uix.textinput import TextInput
from kivy.uix.gridlayout import GridLayout

# Import from other Anya modules
from anya_core.wallet import key_management, transaction, balance, address_management
from anya_core.network import bitcoin_client

# Import Gemini client
import gemini

# Initialize Gemini client 
gemini_client = gemini.PublicClient()

class AnyaWalletApp(App):
    def build(self):
        layout = BoxLayout(orientation='vertical')

        # Balance and Address display
        self.balance_label = Label(text='Balance: 0 BTC')
        self.address_label = Label(text='Address: ')
        layout.add_widget(self.balance_label)
        layout.add_widget(self.address_label)

        # Buttons for main actions
        button_layout = GridLayout(cols=2, spacing=10, padding=10)
        self.send_button = Button(text='Send')
        self.receive_button = Button(text='Receive')
        self.create_wallet_button = Button(text='Create Wallet')
        self.load_wallet_button = Button(text='Load Wallet')
        self.insights_button = Button(text='Get Insights')
        button_layout.add_widget(self.send_button)
        button_layout.add_widget(self.receive_button)
        button_layout.add_widget(self.create_wallet_button)
        button_layout.add_widget(self.load_wallet_button)
        button_layout.add_widget(self.insights_button)
        layout.add_widget(button_layout)

        # Bind button actions
        self.send_button.bind(on_press=self.send_transaction)
        self.receive_button.bind(on_press=self.receive_payment)
        self.create_wallet_button.bind(on_press=self.create_wallet)
        self.load_wallet_button.bind(on_press=self.load_wallet)
        self.insights_button.bind(on_press=self.get_financial_insights)

        # Initialize wallet data
        self.wallets = []
        self.current_wallet = None
        self.user_opted_in_to_ai_insights = False 

        return layout

    def create_wallet(self, instance):
        """
        Handles the creation of a new wallet.
        """

        mnemonic = key_management.generate_mnemonic()

        # Prompt user for a secure passphrase (using a Popup)
        passphrase_input = TextInput(multiline=False, password=True)
        popup_content = BoxLayout(orientation='vertical')
        popup_content.add_widget(Label(text='Enter a secure passphrase:'))
        popup_content.add_widget(passphrase_input)
        save_button = Button(text='Save')
        popup_content.add_widget(save_button)

        popup = Popup(title='Create Wallet', content=popup_content, size_hint=(None, None), size=(400, 200))

        def save_passphrase(instance):
            passphrase = passphrase_input.text
            if not passphrase:
                self._show_error_popup("Passphrase cannot be empty")
            else:
                popup.dismiss()
                self._create_wallet_with_passphrase(mnemonic, passphrase)

        save_button.bind(on_press=save_passphrase)
        popup.open()

    def _create_wallet_with_passphrase(self, mnemonic, passphrase):
        try:
            master_key = key_management.derive_key_from_mnemonic(mnemonic, passphrase)

            # Derive the first receiving address 
            first_address = address_management.generate_new_address(master_key.derive_path("m/84h/0h/0h/0/0"), address_type='p2wpkh')

            wallet_data = {
                'mnemonic': mnemonic,
                'passphrase': passphrase,  
                'master_key': master_key,
                'addresses': [{'address': first_address, 'derivation_path': "m/84h/0h/0h/0/0"}],
                'transactions': [] 
            }
            self.wallets.append(wallet_data)
            self.current_wallet = wallet_data

            self.address_label.text = f"Address: {first_address}"
            self.get_balance() 

            self._show_info_popup("Important!", "Please back up your mnemonic phrase in a safe place. It is the only way to recover your wallet.")
        except ValueError as e: 
            self._show_error_popup(str(e))

    def load_wallet(self, instance):
        """Handles loading an existing wallet."""

        # Prompt user for mnemonic and passphrase (using a Popup)
        mnemonic_input = TextInput(multiline=False)
        passphrase_input = TextInput(multiline=False, password=True)
        popup_content = BoxLayout(orientation='vertical')
        popup_content.add_widget(Label(text='Enter your mnemonic:'))
        popup_content.add_widget(mnemonic_input)
        popup_content.add_widget(Label(text='Enter your passphrase:'))
        popup_content.add_widget(passphrase_input)
        load_button = Button(text='Load')
        popup_content.add_widget(load_button)

        popup = Popup(title='Load Wallet', content=popup_content, size_hint=(None, None), size=(400, 300))

        def load_wallet_data(instance):
            mnemonic = mnemonic_input.text
            passphrase = passphrase_input.text
            if not mnemonic or not passphrase:
                return  # User cancelled or provided invalid input

            if key_management.is_valid_mnemonic(mnemonic):
                try:
                    master_key = key_management.derive_key_from_mnemonic(mnemonic, passphrase)

                    # Derive addresses 
                    addresses = [
                        {
                            'address': address_management.generate_new_address(master_key.derive_path(f"m/84h/0h/0h/0/{i}"), address_type='p2wpkh'), 
                            'derivation_path': f"m/84h/0h/0h/0/{i}"
                        }
                        for i in range(5) 
                    ]

                    # Create wallet representation
                    wallet_data = {
                        'mnemonic': mnemonic,
                        'passphrase': passphrase, 
                        'master_key': master_key,
                        'addresses': addresses,
                        'transactions': [] 
                    }
                    self.wallets.append(wallet_data)
                    self.current_wallet = wallet_data

                    popup.dismiss()

                    # Update UI 
                    self.address_label.text = f"Address: {addresses[0]['address']}" 
                    self.get_balance()
                except Exception as e:
                    self._show_error_popup(f"Error loading wallet: {e}")
            else:
                self._show_error_popup("Invalid mnemonic phrase")

        load_button.bind(on_press=load_wallet_data)
        popup.open()

    def get_balance(self):
        """
        Displays the current wallet's balance, including Bitcoin and Taproot assets (when supported).
        """

        if self.current_wallet:
            try:
                btc_balance = 0
                for address_data in self.current_wallet['addresses']:
                    btc_balance += balance.get_balance(address_data['address'])
