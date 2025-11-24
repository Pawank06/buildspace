# Learning Path - 20+ Native Solana Programs

## Breakdown

- Phase 1 - Base Layer (Month 1-2)
- Phase 2 - Mid-Level Programs (Month 3-4)
- Phase 3 - Advanced Protocols (Month 5-7)
- Phase 4 - God-Level Engineering (Month 8-12)

Each project includes:
- Prerequisite Concepts
- Project Deliverables
- Concepts Learned

---

## Phase 1 - Month 1-2 (Fundamentals + 6 Beginner Projects)

Goal: Become comfortable with native Rust, accounts, PDA, CPI, system program.

### Project 1 - Hello Account Program

A program that initializes an account and writes data into it.

**Before You Start - Learn These Concepts**
- Solana account model
- Data serialization (Borsh)
- system_instruction::create_account
- Program-owned vs system-owned accounts
- Rent-exemption basics
- Solana program entrypoint

**What You Build**
- Initialize a simple state account
- Write/update a value

**Concepts Learned**
- Account ownership rules
- Deserialization by hand
- Instruction unpacking
- System program CPI
- Error handling

### Project 2 - PDA Counter

PDA-owned "counter" that increments only by the creator.

**Before You Start - Learn**
- What is a PDA (with seeds + bump)
- invoke_signed
- Seeds collision
- Signer authority model

**You Build**
- Create PDA via program
- Increment value
- Only creator allowed

**You Learn**
- PDA security
- Canonical seed rules
- Instruction accounts validation
- Sealevel parallelism constraints

### Project 3 - Lamport Vault

A vault PDA that stores SOL and only withdraws to owner.

**Before You Start**
- Lamport transfers
- SystemProgram::transfer
- Ownership vs "authority"
- Custom errors

**Build**
- deposit()
- withdraw()

**Learn**
- invoke_signed for lamport movements
- PDA as SOL vault authority
- Preventing unauthorized withdrawal

### Project 4 - SPL Token Mint Program

Mint tokens using native Rust CPI.

**Before You Start**
- spl-token program
- Associated token accounts
- Mint authority
- Token decimals / supply model

**Build**
- Create mint
- Mint tokens to user
- Freeze/thaw

**Learn**
- Token CPI
- AssociatedTokenAccount CPI
- Secure authority checks

### Project 5 - Basic Escrow (1-shot trade)

Classic escrow for token-for-token exchange.

**Before You Start**
- Token accounts anatomy
- Account closing mechanics
- Time-based sysvars
- Vault pattern

**Build**
- seller deposits
- buyer accepts
- cancellation

**Learn**
- Two-party authority
- Time-based constraints
- Atomic settlement through CPI

### Project 6 - Multisig Wallet (2-of-3)

Your own version of a Gnosis safe.

**Before You Start**
- Multi-signature logic
- Instruction packaging
- Replay protection
- PDA with multiple owners

**Build**
- Add owners
- Add proposal
- Execute only with 2 signers

**Learn**
- Anti-replay security
- Checking signer arrays
- Robust authority systems

---

## Phase 2 - Month 3-4 (Intermediate Protocol Level)

### Project 7 - Staking Program (simple)

Stake SPL tokens and earn rewards.

**Before**
- Clock sysvar
- Rewards math
- CPI transfer rules
- PDA per user

**Build**
- stake()
- claim_rewards()
- unstake()

**Learn**
- Reward-time calculations
- State structs for user + pool
- PDA metadata patterns

### Project 8 - NFT Minting Program (Native)

Use Metaplex metadata CPI.

**Before**
- Metadata accounts
- Master edition
- What is a creator?
- Verified creators

**Build**
- mint_nft()
- update_metadata()

**Learn**
- Cross-program CPI
- Token metadata schemas
- Freeze authority patterns

### Project 9 - Token Vesting (Linear + Cliff)

Token release schedule.

**Before**
- Linear math
- Unix timestamp calculations
- Rent + realloc
- Secure authority retention

**Build**
- create_vesting()
- claim()
- revoke()

**Learn**
- Precise arithmetic
- State lifetime management
- Custom schedule logic

### Project 10 - Cross-Program Router (mini-Jupiter)

Route swaps between Raydium, Orca, etc.

**Before**
- CPI chaining
- Error bubbling
- Token account state reading
- Dynamic instruction dispatch

**Build**
- route() that chooses best AMM
- logs slippage

**Learn**
- Multi-CPI design
- Router patterns
- Program communication

### Project 11 - Liquid Staking (mini-Marinade)

stake SOL and get stSOL

**Before**
- Stake program CPI
- Epoch schedule
- Rewards mechanics
- Delegation

**Build**
- deposit_sol()
- mint_lst()
- redeem_lst()

**Learn**
- Stake account internals
- Inflation rewards
- SOL wrapping

### Project 12 - On-Chain Orderbook (CLOB)

Like Serum/Phoenix.

**Before**
- Slab data structure
- Zero-copy (bytemuck)
- Account paging

**Build**
- place_bid()
- place_ask()
- match_orders()

**Learn**
- Efficient account storage
- Sorting + matching logic
- Real-time trading engine

### Project 13 - AMM / DEX (Uniswap v2 style)

Constant product + LP shares.

**Before**
- k = x*y invariant
- Fee models
- Impermanent loss math
- Pool vault patterns

**Build**
- swap()
- add_liquidity()
- remove_liquidity()

**Learn**
- Advanced token math
- Multiple vault ownership
- Optimal trade path logic

---

## Phase 3 - Month 5-7 (Advanced Protocol Engineering)

### Project 14 - Flash Loan Program

Borrow, use, and repay in same tx.

**Before**
- tx-level atomicity
- Instruction introspection
- Reentrancy prevention
- Fee schedules

**Build**
- borrow()
- repay()
- enforce same-tx return

**Learn**
- Advanced security
- Inner instruction inspection
- Transaction guarantees

### Project 15 - Lending Protocol (Aave-like)

Borrow/lend/health factors.

**Before**
- Interest rate models
- Collateral factor
- Health score math
- Liquidations

**Build**
- supply()
- borrow()
- repay()
- liquidate()

**Learn**
- Complex risk math
- Multi-account interactions
- Time-dependent logic

### Project 16 - Perpetual Futures (Drift-like)

On-chain leveraged trading.

**Before**
- Funding payments
- Mark price feeding (oracle)
- Position entry logic
- Liquidation rules

**Build**
- open_position()
- settle_funding()
- liquidate()

**Learn**
- Oracle CPI
- Derivatives math
- Risk engine internals

### Project 17 - MEV Bundle Executor

Like Jito.

**Before**
- Transactions packing
- Compute unit markets
- Bundle structure
- Censorship resistance

**Build**
- submit_bundle()
- validate_bundle()
- reward logic

**Learn**
- High-performance execution
- Bundle simulation
- MEV strategies

### Project 18 - Rollup Settlement Contract

L2 to Solana bridging.

**Before**
- Merkle proofs
- Verification logic
- State commitments
- Withdrawal challenges

**Build**
- submit_state_root()
- prove_inclusion()

**Learn**
- ZK-friendly layout
- SHA/Merkle hashing
- Bridge design

---

## Phase 4 - Month 8-12 (God-Tier Solana Engineer)

### Project 19 - On-Chain VDF (Verifiable Delay Function)

Compute-bound crypto.

**Before**
- Modular exponentiation
- Cryptographic proof systems
- Time-delay proofs

**Build**
- vdf_init()
- verify()

**Learn**
- Cryptographic primitives
- Execution constraints
- Compute-heavy Rust optimization

### Project 20 - Build a Mini Runtime Inside a Program (Final Boss)

A "VM inside Solana".

**Before**
- RISC design
- Interpreter loops
- State transitions
- Parallel conflict resolution

**Build**
- execute_subtx()
- sub-account scheduler
- deterministic state machine

**Learn**
- Runtime architecture
- Advanced systems engineering
- Low-level Solana internals (banking stage concepts)

---

## What's Next?

By completing these 20 projects, you'll have built a comprehensive understanding of Solana's architecture from the ground up. You'll understand how native programs work without framework abstractions, giving you the knowledge to build production-grade protocols, contribute to major DeFi projects, and architect complex blockchain systems.

Each program in this repository is fully documented with tests, deployment instructions, and detailed explanations. Start with Phase 1 and work your way up - or jump to any project that interests you. The key is to build, break things, and learn from the process.

Welcome to the journey of becoming a Solana protocol engineer.
