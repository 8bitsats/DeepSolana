Training on Solana

Training a 1B-parameter model on-chain is impractical. Instead, we use Solana to coordinate off-chain training and track contributions.

Decentralized Training Workflow:

Initialize Training Job:

Create a Solana program to manage training jobs, incentives, and model updates.

Users stake tokens to participate in training.

Off-Chain Training:

Use decentralized compute networks (e.g., Akash Network, Golem) to train the model.

Fetch the base model from IPFS/Arweave.

Split training tasks across nodes and aggregate results.

Update Model on IPFS/Arweave:

Upload the trained model to decentralized storage.

Update the CID on Solana to point to the new model version.
