"""
Lighthouse Storage Manager for BAAI Node
Handles encrypted data storage and retrieval
"""

from lighthouseweb3 import Lighthouse
from typing import Dict, Optional
import json
import base64
from cryptography.fernet import Fernet

class LighthouseManager:
    def __init__(self, api_key: str):
        self.lighthouse = Lighthouse(token=api_key)
        self.encryption_key = Fernet.generate_key()
        self.fernet = Fernet(self.encryption_key)
        
    async def store_engagement_data(self, data: Dict, proof: Dict) -> str:
        """Store encrypted engagement data and proof"""
        # Combine data and proof
        storage_package = {
            "engagement_data": data,
            "proof": proof,
            "metadata": {
                "version": "1.0",
                "type": "BAAI_ENGAGEMENT",
                "encryption": "fernet"
            }
        }
        
        # Encrypt the package
        encrypted_data = self.encrypt_data(storage_package)
        
        # Upload to Lighthouse
        result = self.lighthouse.upload(
            source=encrypted_data,
            encryption=True
        )
        
        if 'data' in result and 'Hash' in result['data']:
            return result['data']['Hash']
        raise Exception("Failed to upload to Lighthouse")
    
    def encrypt_data(self, data: Dict) -> bytes:
        """Encrypt data using Fernet"""
        json_data = json.dumps(data)
        return self.fernet.encrypt(json_data.encode())
    
    async def retrieve_data(self, cid: str) -> Optional[Dict]:
        """Retrieve and decrypt data from Lighthouse"""
        try:
            response = await self.lighthouse.download(cid)
            decrypted_data = self.fernet.decrypt(response)
            return json.loads(decrypted_data)
        except Exception as e:
            print(f"Error retrieving data: {e}")
            return None 