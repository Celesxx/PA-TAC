import json
from channels.generic.websocket import AsyncWebsocketConsumer
import logging

logger = logging.getLogger(__name__)

class ProgressConsumer(AsyncWebsocketConsumer):
    async def connect(self):
        logger.info("WebSocket Connected")
        await self.channel_layer.group_add("progress_group", self.channel_name)
        await self.accept()

    async def disconnect(self, close_code):
        logger.info(f"WebSocket Disconnected with close code {close_code}")
        await self.channel_layer.group_discard("progress_group", self.channel_name)

    async def send_progress(self, event):
        logger.info(f"Sending progress")
        message = event['message']
        await self.send(text_data=json.dumps(message))
