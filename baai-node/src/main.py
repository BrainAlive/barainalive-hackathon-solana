"""
Main runner script for BAAI Node Service
"""

import asyncio
import os
from baai_node import BAAINode
from engagement_generator import EngagementSimulator

async def main():
    # Initialize BAAI Node
    api_key = os.getenv("LIGHTHOUSE_API_KEY")
    if not api_key:
        raise ValueError("LIGHTHOUSE_API_KEY environment variable not set")
    
    node = BAAINode(api_key)
    simulator = EngagementSimulator()
    
    print("=== Starting BAAI Node Service ===")
    print("Listening for engagement data...")
    
    async def process_engagement(data):
        try:
            cid = await node.process_engagement_stream(data)
            print(f"Processed engagement: {data['engagement_type']}")
            print(f"Stored with CID: {cid}")
        except Exception as e:
            print(f"Error processing engagement: {e}")
    
    # Start streaming simulated engagement data
    await simulator.stream_engagements(process_engagement)

if __name__ == "__main__":
    asyncio.run(main()) 