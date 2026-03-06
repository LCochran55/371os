import numpy as np
from PIL import Image
import requests
from io import BytesIO


#PPM height/width
HEIGHT = 720 
WIDTH = 400

#Grabbing hex values from ppm
IMG_NAME = "binkledump.ppm"

img = np.array(Image.open(IMG_NAME))

HEXVALS = []
for i in range(0,720,45):
    HEXVALS.append(img[0][i])

HEXVALS = np.array(HEXVALS)


#Gets an image by URL
img_url = "https://cd-public.github.io/ai101/images/photo-cat.jpg"
response = requests.get(img_url)
image = Image.open(BytesIO(response.content))

#Turn into a np.array, contains raw pixel values row by row
img_array = np.array(image)

#Resize array to have correct size
img_height = img_array.shape[1]
img_width = img_array.shape[0]

temp_img = [[0 for _ in range(HEIGHT)] for _ in range(WIDTH)]

for r in range(WIDTH):
    for c in range(HEIGHT):
        row = int((img_width*r)/WIDTH)
        col = int((img_height*c)/HEIGHT)
        temp_img[r][c] = img_array[row][col]

scaled_img = np.array(temp_img)

#def color_dif(RGB_1,RGB_2):
 #   red = RGB_1[0] - RGB_2[0]
  #  grn = RGB_1[1] - RGB_2[1]
   # ble = RGB_1[2] - RGB_2[2]

    #r_line = 1/2*(red1 + red2)

    #delta_c = (2 + r_line/256)*red**2 + 4*grn**2 + (2 + (255-r_line)/256)*ble**2
    #delta_c = np.sqrt(delta_c)
    #return delta_c



scaled_img = scaled_img.reshape(-1,3)
diff = np.sum((scaled_img[:,np.newaxis,:] - HEXVALS)**2,axis=-1)

closest_color_indices = np.argmin(diff, axis=1) 
closest_colors = HEXVALS[closest_color_indices]  

color_string = closest_colors.tolist()


with open("src/colors/img.rs", "w", encoding="utf-8") as f:
    f.write("\n".join(str(item) for item in color_string))
