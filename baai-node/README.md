# BAAI Node Service

Unified service for processing engagement data and managing proofs in the BAAI network.

## Features
- Processes engagement analytics from BAAI extension
- Generates ZK proofs for data verification
- Encrypts and stores data on Lighthouse storage
- Manages node configuration and state

## Setup

1. Create virtual environment:
```bash
python -m venv venv
source venv/bin/activate  # or venv\Scripts\activate on Windows
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

3. Configure environment:
```bash
cp config/.env.example config/.env
# Edit config/.env with your settings
```

4. Run the service:
```bash
python baai_node.py
```

## Configuration
Required environment variables:
- `LIGHTHOUSE_API_KEY`: Your Lighthouse storage API key
- `NODE_ID`: Unique identifier for this BAAI node
- `STORAGE_ENCRYPTION_KEY`: Key for encrypting stored data

## Usage
The node service will:
1. Load configuration from environment
2. Initialize Lighthouse storage connection
3. Process incoming engagement data
4. Generate ZK proofs
5. Store encrypted data and proofs 