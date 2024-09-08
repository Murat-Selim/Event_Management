# Solana Event Management DApp

This project is a decentralized application (DApp) built on the Solana blockchain that allows users to create and join events using tokens for ticket payments. The project is implemented using the Anchor framework for smart contract development on Solana.

## Features

- **Create Events**: Organizers can create events with details such as name, description, date, maximum participants, and ticket price.
- **Join Events**: Participants can join events by transferring tokens as payment for tickets.
- **Token Transfer**: Payment for events is handled via token transfer using the SPL Token standard.

## Smart Contract Structure

- **create_event**: Allows an organizer to create an event.
- **join_event**: Allows participants to join an event by paying the event's token price.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html) framework installed.
- A [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) installation.
- An active Solana devnet wallet with enough SOL tokens for testing.

## Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/your-username/event-management-project.git
    cd event-management-project
    ```

2. **Install dependencies**:
    Make sure that `anchor` is installed by running:
    ```bash
    anchor --version
    ```

3. **Build the project**:
    Compile the program with:
    ```bash
    anchor build
    ```

4. **Deploy the smart contract**:
    Deploy the program to Solana Devnet:
    ```bash
    anchor deploy
    ```

5. **Run tests**:
    Test the program to ensure functionality:
    ```bash
    anchor test
    ```

## Program Details

- **Program ID**: `3JeL9vwp7EFB2xmfW4BAyU4VpGL5Fw3nPz9xi8oNm1M4`
- **Associated Accounts**:
  - Event Account: Stores event details such as organizer, name, description, participants, and ticket price.
  - Participant Account: Used for storing tokens for event participation.
  - Token Mint: The SPL Token used for transactions in the DApp.
