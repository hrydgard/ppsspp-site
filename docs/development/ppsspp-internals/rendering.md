# Rendering

Just various notes, and answered questions.

## Bones/skinning

You won't see it in the kernel APIs directly, however, bones are sent as commands in display lists (which are kicked off with `sceGeListEnqueue`). Games use `GE_CMD_BONEMATRIXNUMBER` / `GE_CMD_BONEMATRIXDATA` repeatedly to upload bone matrices into registers, then if the vertex format contains bone weights, skinning is applied accordingly.

Note that it's a fixed matrix palette of 8 matrices, so if you need more (and you almost always do) you need to split the model into multiple draws, and upload new matrices in between. Modern skinning instead has, for example, 4 weights and 4 matrix indices in each vertex, so each vertex can make use of up to four out of a large number of matrices.