# Fast3DBuffer
Fast3DBuffer is a type in SM64Lib (the backbone of SM64 Rom Manager) which contains data about a level's visual model such as lighting, textures, and Fast3D display lists.

Fast3DBuffers are made of multiple parts:
- [Lighting Information](#Lighting)
- [Materials](#Materials)

## Lighting
- 00: Ambient Color R
- 01: Ambient Color G
- 02: Ambient Color B
- 03: unused, always 0
- 04: Ambient Color R again
- 05: Ambient Color G again
- 06: Ambient Color B again
- 07: unused, always 0 again

- 08: Diffuse Color R
- 09: Diffuse Color G
- 10: Diffuse Color B
- 11: unused, always 0
- 12: Diffuse Color R again
- 13: Diffuse Color G again
- 14: Diffuse Color B again
- 15: unused, always 0 again

- 16: Diffuse Light Direction X (default 0x49)
- 17: Diffuse Light Direction Y (default 0x49)
- 18: Diffuse Light Direction Z (default 0x49)

- 19-23: unused, always 0

## Materials
idk
