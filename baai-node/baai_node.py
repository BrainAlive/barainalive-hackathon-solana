"""
BAAI Node Service
Unified service for processing engagement data, managing proofs, and storage
"""

import os
import asyncio
import json
from dataclasses import dataclass
from typing import Dict, Optional, List
from lighthouseweb3 import Lighthouse
from cryptography.fernet import Fernet
from dotenv import load_dotenv

@dataclass
class NodeConfig:
    node_id: str
    lighthouse_api_key: str
    storage_encryption_key: str

class BAAINode:
    def __init__(self):
        self.config = self._load_config()
        self.lighthouse = Lighthouse(token=self.config.lighthouse_api_key)
        self.fernet = Fernet(self.config.storage_encryption_key.encode())
        print(f"=== Starting BAAI Node {self.config.node_id} ===")
        
    def _load_config(self) -> NodeConfig:
        """Load configuration from environment variables"""
        load_dotenv()
        
        required_vars = [
            "LIGHTHOUSE_API_KEY",
            "NODE_ID",
            "STORAGE_ENCRYPTION_KEY"
        ]
        
        missing = [var for var in required_vars if not os.getenv(var)]
        if missing:
            raise ValueError(f"Missing required environment variables: {', '.join(missing)}")
            
        return NodeConfig(
            node_id=os.getenv("NODE_ID"),
            lighthouse_api_key=os.getenv("LIGHTHOUSE_API_KEY"),
            storage_encryption_key=os.getenv("STORAGE_ENCRYPTION_KEY")
        )

    async def process_engagement(self, engagement_data: Dict) -> Optional[str]:
        """Process engagement data and store with proof"""
        try:
            # Generate proof
            proof = self._generate_proof(engagement_data)
            if not proof:
                raise Exception("Failed to generate proof")
                
            # Store data and proof
            cid = await self._store_data(engagement_data, proof)
            print(f"Processed engagement data. CID: {cid}")
            return cid
            
        except Exception as e:
            print(f"Error processing engagement: {e}")
            return None

    def _generate_proof(self, data: Dict) -> Optional[Dict]:
        """Generate ZK proof for engagement data"""
        try:
            # TODO: Integrate with EZKL library
            # Mock proof generation for now
            proof = {
                "proof": "mock_proof",
                "public_inputs": [
                    data.get("engagement_type"),
                    str(data.get("timestamp"))
                ],
                "verification_key": "mock_key"
            }
            return proof
        except Exception as e:
            print(f"Error generating proof: {e}")
            return None

    async def _store_data(self, data: Dict, proof: Dict) -> str:
        """Store encrypted data and proof on Lighthouse"""
        storage_package = {
            "engagement_data": data,
            "proof": proof,
            "metadata": {
                "node_id": self.config.node_id,
                "version": "1.0",
                "type": "BAAI_ENGAGEMENT"
            }
        }
        
        # Encrypt the package
        encrypted_data = self._encrypt_data(storage_package)
        
        # Upload to Lighthouse
        result = self.lighthouse.upload(
            source=encrypted_data,
            encryption=True
        )
        
        if 'data' in result and 'Hash' in result['data']:
            return result['data']['Hash']
        raise Exception("Failed to upload to Lighthouse")

    def _encrypt_data(self, data: Dict) -> bytes:
        """Encrypt data using Fernet"""
        json_data = json.dumps(data)
        return self.fernet.encrypt(json_data.encode())

    async def run(self):
        """Main service loop"""
        print("Ready to process engagement data...")
        try:
            while True:
                # Wait for incoming data
                await asyncio.sleep(1)
        except KeyboardInterrupt:
            print("\nShutting down BAAI Node...")

async def main():
    node = BAAINode()
    await node.run()

if __name__ == "__main__":
    asyncio.run(main()) 