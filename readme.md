# Ledger Lessons

Solana Programs powered by Anchor.

## Ideas

A mini deposit-refund system for Encode Club that allows users to deposit 250 USDC (from whatever platform they are using) and receive an automatic refund if certain conditions are met. 

The refund condition would be a minimum 90% attendence time. The authorized Encode Club accounts would be able to approve the refund...etc. 

1) Smart Contract for Deposit and Refund: Manages the logic for depositing USDC, tracking attendance, and processing refunds.

2) Frontend Application: Interfaces with the smart contract for users and staff. Allows users to deposit USDC and view their attendance and refund status. Allows authorized accounts to verify and approve refunds.

3) Oracles or Integrations for Attendance Tracking: Reliable tracking and recording of attendance data which would likely need to interface with the educational platform's backend. (Not too familiar with oracles though)