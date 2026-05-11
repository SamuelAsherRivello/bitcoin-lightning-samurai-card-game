from pathlib import Path
from random import Random

from PIL import Image, ImageChops, ImageDraw, ImageFilter


WIDTH = 840
HEIGHT = 1440
SCALE = 2
ROOT = Path(__file__).resolve().parents[2]
THEME_CARDS = (
    ROOT / "bevy" / "crates" / "game" / "assets" / "themes" / "theme_japan" / "cards"
)


def scaled(points):
    return [(int(x * SCALE), int(y * SCALE)) for x, y in points]


def canvas():
    return Image.new("RGBA", (WIDTH * SCALE, HEIGHT * SCALE), (0, 0, 0, 0))


def downsample(image):
    return image.resize((WIDTH, HEIGHT), Image.Resampling.LANCZOS)


def jittered_vertical(x, top, bottom, wobble, steps, rng):
    return [(x + rng.randint(-wobble, wobble), y) for y in range(top, bottom + 1, (bottom - top) // steps)]


def jittered_horizontal(y, left, right, wobble, steps, rng):
    return [(x, y + rng.randint(-wobble, wobble)) for x in range(left, right + 1, (right - left) // steps)]


def draw_polyline(draw, points, fill, width, joint="curve"):
    draw.line(scaled(points), fill=fill, width=width * SCALE, joint=joint)


def draw_leaf(draw, x, y, angle, fill):
    size = 10 * SCALE
    cx = int(x * SCALE)
    cy = int(y * SCALE)
    pts = [
        (cx, cy - size),
        (cx + int(size * 0.7), cy),
        (cx, cy + size),
        (cx - int(size * 0.7), cy),
    ]
    draw.polygon(pts, fill=fill)


def frame_mask(outer, inner):
    outer_mask = Image.new("L", (WIDTH * SCALE, HEIGHT * SCALE), 0)
    inner_mask = Image.new("L", (WIDTH * SCALE, HEIGHT * SCALE), 0)
    d_outer = ImageDraw.Draw(outer_mask)
    d_inner = ImageDraw.Draw(inner_mask)
    d_outer.polygon(scaled(outer), fill=255)
    d_inner.polygon(scaled(inner), fill=255)
    return ImageChops.subtract(outer_mask, inner_mask).filter(ImageFilter.GaussianBlur(0.45 * SCALE))


def apply_mask(texture, mask):
    texture.putalpha(mask)
    return texture


def save(image, folder):
    path = THEME_CARDS / folder / "frame.png"
    path.parent.mkdir(parents=True, exist_ok=True)
    downsample(image).save(path)
    print(path.relative_to(ROOT))


def make_kage_ren():
    rng = Random(17)
    img = canvas()
    d = ImageDraw.Draw(img, "RGBA")
    bamboo = (29, 37, 24, 242)
    bamboo_dark = (12, 18, 12, 238)
    highlight = (63, 76, 45, 115)
    vine = (35, 72, 38, 230)
    vine_light = (75, 122, 67, 205)

    left = jittered_vertical(64, 60, 1380, 10, 18, rng)
    right = jittered_vertical(776, 58, 1382, 10, 18, rng)
    top = jittered_horizontal(66, 66, 774, 8, 12, rng)
    bottom = jittered_horizontal(1374, 66, 774, 8, 12, rng)

    for pts in (left, right, top, bottom):
        draw_polyline(d, pts, bamboo_dark, 62)
        draw_polyline(d, pts, bamboo, 46)
        draw_polyline(d, [(x - 7, y - 4) for x, y in pts], highlight, 8)

    for y in range(150, 1320, 170):
        d.line(scaled([(44, y + rng.randint(-9, 9)), (83, y + rng.randint(-9, 9))]), fill=(8, 12, 8, 150), width=4 * SCALE)
        d.line(scaled([(757, y + rng.randint(-9, 9)), (796, y + rng.randint(-9, 9))]), fill=(8, 12, 8, 150), width=4 * SCALE)
    for x in range(150, 720, 150):
        d.line(scaled([(x + rng.randint(-8, 8), 46), (x + rng.randint(-8, 8), 86)]), fill=(8, 12, 8, 145), width=4 * SCALE)
        d.line(scaled([(x + rng.randint(-8, 8), 1355), (x + rng.randint(-8, 8), 1394)]), fill=(8, 12, 8, 145), width=4 * SCALE)

    corners = [(62, 64), (778, 64), (62, 1376), (778, 1376)]
    for cx, cy in corners:
        direction = -1 if cx > WIDTH / 2 else 1
        ydir = -1 if cy > HEIGHT / 2 else 1
        vine_pts = [(cx, cy), (cx + 36 * direction, cy + 24 * ydir), (cx + 20 * direction, cy + 64 * ydir), (cx + 66 * direction, cy + 94 * ydir)]
        draw_polyline(d, vine_pts, vine, 8)
        for px, py in vine_pts[1:]:
            draw_leaf(d, px + rng.randint(-6, 6), py + rng.randint(-5, 5), 0, vine_light)

    return img


def make_lord_daichi():
    rng = Random(23)
    tex = Image.new("RGBA", (WIDTH * SCALE, HEIGHT * SCALE), (43, 27, 17, 0))
    d = ImageDraw.Draw(tex, "RGBA")
    outer = [(42, 42), (798, 42), (798, 1398), (42, 1398)]
    inner = [(96, 96), (744, 96), (744, 1344), (96, 1344)]
    mask = frame_mask(outer, inner)
    d.rounded_rectangle([42 * SCALE, 42 * SCALE, 798 * SCALE, 1398 * SCALE], radius=18 * SCALE, fill=(42, 27, 17, 245))
    d.rounded_rectangle([96 * SCALE, 96 * SCALE, 744 * SCALE, 1344 * SCALE], radius=7 * SCALE, fill=(0, 0, 0, 0))

    for i in range(95):
        if i % 2 == 0:
            y = rng.randint(45, 1395)
            d.line([(44 * SCALE, y * SCALE), (796 * SCALE, (y + rng.randint(-8, 8)) * SCALE)], fill=(85, 56, 31, rng.randint(28, 72)), width=rng.randint(1, 3) * SCALE)
        x = rng.randint(44, 796)
        d.line([(x * SCALE, 44 * SCALE), ((x + rng.randint(-8, 8)) * SCALE, 1396 * SCALE)], fill=(18, 12, 9, rng.randint(18, 46)), width=1 * SCALE)
    d.rounded_rectangle([42 * SCALE, 42 * SCALE, 798 * SCALE, 1398 * SCALE], radius=18 * SCALE, outline=(96, 66, 36, 155), width=5 * SCALE)
    d.rounded_rectangle([96 * SCALE, 96 * SCALE, 744 * SCALE, 1344 * SCALE], radius=7 * SCALE, outline=(15, 10, 7, 185), width=4 * SCALE)
    return apply_mask(tex, mask)


def make_sister_hotaru():
    rng = Random(41)
    tex = Image.new("RGBA", (WIDTH * SCALE, HEIGHT * SCALE), (0, 0, 0, 0))
    d = ImageDraw.Draw(tex, "RGBA")
    outer = [(48, 55), (787, 48), (792, 1386), (51, 1392)]
    inner = [(111, 111), (730, 102), (721, 1331), (105, 1338)]
    mask = frame_mask(outer, inner)
    d.polygon(scaled(outer), fill=(37, 38, 43, 242))
    d.polygon(scaled(inner), fill=(0, 0, 0, 0))

    for y in range(80, 1370, 42):
        wave = [(48, y), (180, y + rng.randint(-13, 13)), (420, y + rng.randint(-10, 10)), (660, y + rng.randint(-13, 13)), (790, y + rng.randint(-10, 10))]
        draw_polyline(d, wave, (18, 19, 23, 42), 3)
    for x in (73, 767):
        for y in range(130, 1300, 92):
            d.arc([int((x - 17) * SCALE), int((y - 22) * SCALE), int((x + 17) * SCALE), int((y + 22) * SCALE)], 210, 510, fill=(174, 133, 54, 178), width=3 * SCALE)
            d.ellipse([int((x - 4) * SCALE), int((y - 4) * SCALE), int((x + 4) * SCALE), int((y + 4) * SCALE)], fill=(211, 171, 81, 210))
    for x in range(155, 705, 88):
        d.line(scaled([(x, 70), (x + 26, 86), (x + 52, 70)]), fill=(195, 151, 64, 145), width=3 * SCALE)
        d.line(scaled([(x, 1370), (x + 26, 1354), (x + 52, 1370)]), fill=(195, 151, 64, 145), width=3 * SCALE)
    return apply_mask(tex, mask)


def make_yokai_placeholder():
    rng = Random(73)
    tex = Image.new("RGBA", (WIDTH * SCALE, HEIGHT * SCALE), (0, 0, 0, 0))
    d = ImageDraw.Draw(tex, "RGBA")
    outer = [(41, 41), (804, 32), (793, 1408), (35, 1395)]
    inner = [(104, 118), (724, 91), (710, 1322), (111, 1352)]
    mask = frame_mask(outer, inner)
    d.polygon(scaled(outer), fill=(45, 32, 27, 244))
    d.polygon(scaled(inner), fill=(0, 0, 0, 0))

    for side_x in (65, 775):
        for y in range(105, 1375, 86):
            width = rng.randint(34, 52)
            x = side_x + rng.randint(-7, 7)
            d.rounded_rectangle(
                [int((x - width / 2) * SCALE), int((y - 33) * SCALE), int((x + width / 2) * SCALE), int((y + 34) * SCALE)],
                radius=18 * SCALE,
                fill=(62, 43, 34, 230),
                outline=(20, 14, 12, 150),
                width=3 * SCALE,
            )
            d.line(scaled([(x - width / 2 + 5, y + 28), (x + width / 2 - 5, y - 28)]), fill=(105, 72, 52, 130), width=4 * SCALE)
    for x in range(95, 760, 92):
        for y in (62, 1377):
            d.rounded_rectangle([int((x - 42) * SCALE), int((y - 24) * SCALE), int((x + 42) * SCALE), int((y + 24) * SCALE)], radius=18 * SCALE, fill=(55, 38, 31, 228), outline=(18, 13, 12, 150), width=3 * SCALE)
            d.arc([int((x - 34) * SCALE), int((y - 18) * SCALE), int((x + 34) * SCALE), int((y + 18) * SCALE)], 8, 172, fill=(113, 78, 54, 112), width=4 * SCALE)
    for _ in range(85):
        x = rng.randint(45, 795)
        y = rng.randint(45, 1395)
        d.line(scaled([(x, y), (x + rng.randint(-16, 16), y + rng.randint(-16, 16))]), fill=(118, 83, 58, rng.randint(36, 88)), width=rng.randint(1, 3) * SCALE)
    return apply_mask(tex, mask)


def main():
    save(make_kage_ren(), "card_kage_ren")
    save(make_lord_daichi(), "card_lord_daichi")
    save(make_sister_hotaru(), "card_sister_hotaru")
    save(make_yokai_placeholder(), "card_yokai_placeholder")


if __name__ == "__main__":
    main()
