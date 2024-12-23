# written by carter. 20241222
# "TURN IT OFF TURN IT OFF TURN IT OFF"
import os

jp = [
    "segment2.071A0.ia1", # A
    "segment2.071B0.ia1", # B
    "segment2.071C0.ia1", # C
    "segment2.071D0.ia1", # D
    "segment2.071E0.ia1", # E
    "segment2.071F0.ia1", # F
    "segment2.07200.ia1", # G
    "segment2.07210.ia1", # H
    "segment2.07220.ia1", # I
    "segment2.07230.ia1", # J
    "segment2.07240.ia1", # K
    "segment2.07250.ia1", # L
    "segment2.07260.ia1", # M
    "segment2.07270.ia1", # N
    "segment2.07280.ia1", # O
    "segment2.07290.ia1", # P
    "segment2.072A0.ia1", # Q
    "segment2.072B0.ia1", # R
    "segment2.072C0.ia1", # S
    "segment2.072D0.ia1", # T
    "segment2.072E0.ia1", # U
    "segment2.072F0.ia1", # V
    "segment2.07300.ia1", # W
    "segment2.07310.ia1", # X
    "segment2.07320.ia1", # Y
    "segment2.07330.ia1", # Z
    "segment2.07350.ia1", # !
    "segment2.07390.ia1", # ?
    "segment2.073B0.ia1", # (
    "segment2.073C0.ia1", # )(
    "segment2.073D0.ia1", # )
    "segment2.077A0.ia1", # ,
    "segment2.07B40.ia1", # ...
]

us = [
    "font_graphics.05B80.ia4", # A
    "font_graphics.05BC0.ia4", # B
    "font_graphics.05C00.ia4", # C
    "font_graphics.05C40.ia4", # D
    "font_graphics.05C80.ia4", # E
    "font_graphics.05CC0.ia4", # F
    "font_graphics.05D00.ia4", # G
    "font_graphics.05D40.ia4", # H
    "font_graphics.05D80.ia4", # I
    "font_graphics.05DC0.ia4", # J
    "font_graphics.05E00.ia4", # K
    "font_graphics.05E40.ia4", # L
    "font_graphics.05E80.ia4", # M
    "font_graphics.05EC0.ia4", # N
    "font_graphics.05F00.ia4", # O
    "font_graphics.05F40.ia4", # P
    "font_graphics.05F80.ia4", # Q
    "font_graphics.05FC0.ia4", # R
    "font_graphics.06000.ia4", # S
    "font_graphics.06040.ia4", # T
    "font_graphics.06080.ia4", # U
    "font_graphics.060C0.ia4", # V
    "font_graphics.06100.ia4", # W
    "font_graphics.06140.ia4", # X
    "font_graphics.06180.ia4", # Y
    "font_graphics.061C0.ia4", # Z
    "font_graphics.068C0.ia4", # !
    "font_graphics.06BC0.ia4", # ?
    "font_graphics.06980.ia4", # (
    "font_graphics.069C0.ia4", # )(
    "font_graphics.06A00.ia4", # )
    "font_graphics.06B40.ia4", # ,
    "font_graphics.06D00.ia4", # ...
]

us_lower = [
    "font_graphics.06200.ia4", # A
    "font_graphics.06240.ia4", # B
    "font_graphics.06280.ia4", # C
    "font_graphics.062C0.ia4", # D
    "font_graphics.06300.ia4", # E
    "font_graphics.06340.ia4", # F
    "font_graphics.06380.ia4", # G
    "font_graphics.063C0.ia4", # H
    "font_graphics.06400.ia4", # I
    "font_graphics.06440.ia4", # J
    "font_graphics.06480.ia4", # K
    "font_graphics.064C0.ia4", # L
    "font_graphics.06500.ia4", # M
    "font_graphics.06540.ia4", # N
    "font_graphics.06580.ia4", # O
    "font_graphics.065C0.ia4", # P
    "font_graphics.06600.ia4", # Q
    "font_graphics.06640.ia4", # R
    "font_graphics.06680.ia4", # S
    "font_graphics.066C0.ia4", # T
    "font_graphics.06700.ia4", # U
    "font_graphics.06740.ia4", # V
    "font_graphics.06780.ia4", # W
    "font_graphics.067C0.ia4", # X
    "font_graphics.06800.ia4", # Y
    "font_graphics.06840.ia4", # Z
]

def full_path(name):
    return f"textures/segment2/{name}.png"

def list_get(l, idx):
  try:
    return l[idx]
  except IndexError:
    return None

for index, filename in enumerate(jp):
    fulljp = full_path(filename)
    fullus = full_path(us[index])
    os.system(f"magick convert {fulljp} -rotate 90 -flip {fullus}")
    if lowername := list_get(us_lower, index):
        os.system(f"cp {fullus} {full_path(lowername)}")
