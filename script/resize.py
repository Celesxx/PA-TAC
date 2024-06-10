import sys
from PIL import Image
import os

def resize_image(input_path, output_path, size):
    with Image.open(input_path) as img:
        resized_img = img.resize(size, Image.Resampling.LANCZOS)
        resized_img.save(output_path)
        print(f"Status : Image resized and saved to {output_path}")

def crop_image(input_path, output_path, size):
    with Image.open(input_path) as img:
        width, height = img.size
        left = (width - size[0])/2
        top = (height - size[1])/2
        right = (width + size[0])/2
        bottom = (height + size[1])/2
        cropped_img = img.crop((left, top, right, bottom))
        cropped_img.save(output_path)
        print(f"Status : Image cropped and saved to {output_path}")

def process_images(folder_path, dest_folder, effect, size):
    os.makedirs(dest_folder, exist_ok=True)  # Create destination folder if it does not exist
    print(f"Status : Processing images from {folder_path} to {dest_folder} using {effect}")
    files_processed = 0
    for filename in os.listdir(folder_path):
        if filename.lower().endswith(('.png', '.jpg', '.jpeg')):
            input_path = os.path.join(folder_path, filename)
            output_path = os.path.join(dest_folder, f"{filename}")
            if effect == 'resize':
                resize_image(input_path, output_path, size)
            elif effect == 'crop':
                crop_image(input_path, output_path, size)
            files_processed += 1
    print(f"Status : otal files processed: {files_processed}")

if __name__ == "__main__":
    if len(sys.argv) != 6:
        print("Status : Use the following format")
        print("python script.py <resize or crop> <width> <height> <source_folder> <destination_folder>")
        sys.exit(1)

    effect = sys.argv[1].lower()
    if effect not in ['resize', 'crop']:
        print("Status : effect must be 'resize' or 'crop'")
        sys.exit(1)

    width = int(sys.argv[2])
    height = int(sys.argv[3])
    source_folder = sys.argv[4]
    destination_folder = sys.argv[5]

    process_images(source_folder, destination_folder, effect, (width, height))
