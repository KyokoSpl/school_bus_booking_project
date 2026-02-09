from PIL import Image, ImageDraw, ImageFont

size = 1024
img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

margin = 40
draw.ellipse([margin, margin, size-margin, size-margin], fill='#F59E0B')
draw.ellipse([margin+20, margin+20, size-margin-20, size-margin-20], fill='#FBBF24')

bx, by = 250, 340
bw, bh = 524, 280
draw.rounded_rectangle([bx, by, bx+bw, by+bh], radius=30, fill='#1E40AF')

for i in range(4):
    wx = bx + 40 + i * 125
    wy = by + 30
    draw.rounded_rectangle([wx, wy, wx+100, wy+110], radius=12, fill='#BFDBFE')

draw.rounded_rectangle([bx+bw-140, by+20, bx+bw-20, by+180], radius=15, fill='#93C5FD')

draw.ellipse([bx+60, by+bh-40, bx+160, by+bh+60], fill='#1F2937')
draw.ellipse([bx+80, by+bh-20, bx+140, by+bh+40], fill='#6B7280')
draw.ellipse([bx+bw-180, by+bh-40, bx+bw-80, by+bh+60], fill='#1F2937')
draw.ellipse([bx+bw-160, by+bh-20, bx+bw-100, by+bh+40], fill='#6B7280')

draw.rounded_rectangle([bx+10, by+bh-20, bx+bw-10, by+bh+5], radius=5, fill='#1E3A8A')

try:
    font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf', 120)
except Exception:
    font = ImageFont.load_default()

draw.text((size//2, 720), 'BB', fill='#1E3A8A', font=font, anchor='mt')

img.save('app-icon.png')
print('Icon created: app-icon.png')
