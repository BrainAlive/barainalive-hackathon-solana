"""
BAAI Extension Simulator
Generates sample engagement analytics data
"""

import random
import time
from dataclasses import dataclass
from typing import Dict, List

ENGAGEMENT_TYPES = [
    "YAWNING",
    "LOOKING_AWAY",
    "FOCUSED",
    "DISTRACTED",
    "CONFUSED",
    "UNDERSTANDING",
    "TAKING_NOTES",
    "INACTIVE"
]

class EngagementSimulator:
    def __init__(self):
        self.user_id = f"user_{random.randint(1000, 9999)}"
        
    def generate_engagement(self) -> Dict:
        """Generate sample engagement data"""
        engagement_type = random.choice(ENGAGEMENT_TYPES)
        
        metrics = {
            "confidence": random.uniform(0.7, 1.0),
            "duration": random.uniform(1.0, 5.0),
            "intensity": random.uniform(0.1, 1.0)
        }
        
        return {
            "timestamp": int(time.time()),
            "engagement_type": engagement_type,
            "metrics": metrics,
            "user_id": self.user_id
        }

    async def stream_engagements(self, callback, interval=1.0):
        """Simulate streaming engagement data"""
        while True:
            engagement = self.generate_engagement()
            await callback(engagement)
            time.sleep(interval) 