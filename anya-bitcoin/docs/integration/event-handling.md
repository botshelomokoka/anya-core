# Event Handling

Event handling is a crucial part of integrating Anya Core into your application. Events are fired whenever something important happens in the system, such as a new block being found or a transaction being confirmed. By listening to these events, you can build a robust and scalable application that is always up-to-date with the latest state of the blockchain.

## Event Types

The following events are available:

### Block Found

* `BlockFound`: A new block has been found on the blockchain. This event is fired whenever a new block is found, even if it is not yet confirmed.

### Transaction Confirmed

* `TransactionConfirmed`: A transaction has been confirmed on the blockchain. This event is fired whenever a transaction is confirmed, regardless of whether it is a local transaction or not.

### Transaction Received

* `TransactionReceived`: A new transaction has been received by the system. This event is fired whenever a new transaction is received, regardless of whether it is a local transaction or not.

### Balance Changed

* `BalanceChanged`: The balance of a particular user has changed. This event is fired whenever the balance of a user changes, regardless of whether it is due to a local transaction or not.

### User Registered

* `UserRegistered`: A new user has been registered on the system. This event is fired whenever a new user is registered, regardless of whether they are a local user or not.

### User Logged In

* `UserLoggedIn`: A user has logged in to the system. This event is fired whenever a user logs in, regardless of whether they are a local user or not.

### User Logged Out

* `UserLoggedOut`: A user has logged out of the system. This event is fired whenever a user logs out, regardless of whether they are a local user or not.

## Listening to Events

To listen to events, you need to create an event listener and register it with the system. The event listener is a function that is called whenever an event is fired. The event listener should take the event as an argument.
