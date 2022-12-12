import sys
from PIL import Image

if __name__ == "__main__":
    gif = sys.argv[1]
    frame_count = int(sys.argv[2])
    name = sys.argv[3]
    
    with Image.open(gif) as im:
        for i in range(frame_count):
            im.seek(im.n_frames // frame_count * i)
            im.save("./{}/{}_{}.png".format(name, name, i))