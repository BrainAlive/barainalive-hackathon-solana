"""
BAAI Node Service
Core service for processing engagement data and managing proofs
"""

import os
from dataclasses import dataclass
from typing import Dict, Optional
from .lighthouse_manager import LighthouseManager
from .zkml_prover import ZKMLProver

@dataclass
class NodeConfig:
    node_id: str
    lighthouse_api_key: str
    storage_encryption_key: str

class BAAINodeService:
    def __init__(self, config: NodeConfig):
        self.config = config
        self.lighthouse = LighthouseManager(
            api_key=config.lighthouse_api_key,
            encryption_key=config.storage_encryption_key
        )
        self.prover = ZKMLProver()
        
    async def process_engagement(self, engagement_data: Dict) -> Optional[str]:
        """Process engagement data and store with proof"""
        try:
            # Generate proof
            proof = self.prover.generate_proof(engagement_data)
            if not proof:
                raise Exception("Failed to generate proof")
                
            # Store data and proof
            cid = await self.lighthouse.store_engagement_data(
                data=engagement_data,
                proof=proof
            )
            
            return cid
        except Exception as e:
            print(f"Error processing engagement: {e}")
            return None 