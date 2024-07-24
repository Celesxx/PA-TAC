from django.core.management.base import BaseCommand
from django.urls import get_resolver
from django.conf import settings

class Command(BaseCommand):
    help = 'Lists all available URLs, including WebSocket URLs'

    def add_arguments(self, parser):
        parser.add_argument(
            '--ip',
            type=str,
            default='127.0.0.1',
            help='IP address of the server'
        )
        parser.add_argument(
            '--port',
            type=str,
            default='8000',
            help='Port of the server'
        )

    def handle(self, *args, **options):
        ip = options['ip']
        port = options['port']
        base_url = f'http://{ip}:{port}'
        ws_base_url = f'ws://{ip}:{port}'

        resolver = get_resolver()
        urls = resolver.reverse_dict.keys()

        print("HTTP URLs:")
        for url in urls:
            if isinstance(url, str):
                print(f'{base_url}/{url}')

        # Print WebSocket URLs
        from application.routing import websocket_urlpatterns
        print("\nWebSocket URLs:")
        for pattern in websocket_urlpatterns:
            print(f'{ws_base_url}/{pattern.pattern}')
