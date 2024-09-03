"""
This module provides encryption and decryption functionalities for Anya Wallet
"""

from cryptography.fernet import Fernet
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
import os
import base64

def generate_key(password):
    """
    Generates an encryption key from a password using PBKDF2.

    Args:
        password: The password to use for key derivation

    Returns:
        The derived encryption key
    """

    salt = os.urandom(16)
    kdf = PBKDF2HMAC(
        algorithm=hashes.SHA256(),
        length=32,
        salt=salt,
        iterations=390000,  # You can adjust this for stronger security
    )
    key = base64.urlsafe_b64encode(kdf.derive(password.encode()))
    return key

def encrypt_data(data, password):
    """
    Encrypts data using a password-derived key

    Args:
        data: The data to encrypt (bytes)
        password: The password to use for key derivation

    Returns:
        The encrypted data (bytes)
    """

    key = generate_key(password)
    f = Fernet(key)
    encrypted_data = f.encrypt(data)
    return encrypted_data

def decrypt_data(encrypted_data, password):
    """
    Decrypts data using a password-derived key

    Args:
        encrypted_data: The encrypted data (bytes)
        password: The password used for encryption

    Returns:
        The decrypted data (bytes)

    Raises:
        cryptography.fernet.InvalidToken: If decryption fails (e.g. wrong password)
    """

    key = generate_key(password)
    f = Fernet(key)
    decrypted_data = f.decrypt(encrypted_data)
    return decrypted_data

