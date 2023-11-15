# Details

Date : 2023-11-14 08:52:58

Directory e:\\Project\\Rust\\ray_v2

Total : 75 files,  7571 codes, 355 comments, 661 blanks, all 8587 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [.idea/modules.xml](/.idea/modules.xml) | XML | 8 | 0 | 0 | 8 |
| [.idea/ray_v2.iml](/.idea/ray_v2.iml) | XML | 8 | 0 | 0 | 8 |
| [.idea/vcs.xml](/.idea/vcs.xml) | XML | 6 | 0 | 0 | 6 |
| [Cargo.lock](/Cargo.lock) | TOML | 1,268 | 2 | 161 | 1,431 |
| [Cargo.toml](/Cargo.toml) | TOML | 18 | 1 | 5 | 24 |
| [file/box.json](/file/box.json) | JSON | 68 | 0 | 1 | 69 |
| [file/box.toml](/file/box.toml) | TOML | 75 | 18 | 20 | 113 |
| [file/sence.json](/file/sence.json) | JSON | 68 | 0 | 1 | 69 |
| [file/sence.toml](/file/sence.toml) | TOML | 45 | 8 | 10 | 63 |
| [file/setting.toml](/file/setting.toml) | TOML | 17 | 0 | 3 | 20 |
| [file/skybox.toml](/file/skybox.toml) | TOML | 47 | 30 | 21 | 98 |
| [file/sphere.json](/file/sphere.json) | JSON | 68 | 0 | 1 | 69 |
| [object/log/output.log](/object/log/output.log) | Log | 0 | 0 | 1 | 1 |
| [src/main.rs](/src/main.rs) | Rust | 8 | 0 | 6 | 14 |
| [src/pbrt_core/bsdf/mod.rs](/src/pbrt_core/bsdf/mod.rs) | Rust | 0 | 0 | 1 | 1 |
| [src/pbrt_core/bssrdf/mod.rs](/src/pbrt_core/bssrdf/mod.rs) | Rust | 5 | 0 | 1 | 6 |
| [src/pbrt_core/bxdf/disney.rs](/src/pbrt_core/bxdf/disney.rs) | Rust | 57 | 2 | 8 | 67 |
| [src/pbrt_core/bxdf/frensnel.rs](/src/pbrt_core/bxdf/frensnel.rs) | Rust | 107 | 20 | 9 | 136 |
| [src/pbrt_core/bxdf/microfacet.rs](/src/pbrt_core/bxdf/microfacet.rs) | Rust | 0 | 0 | 1 | 1 |
| [src/pbrt_core/bxdf/microfacet_distribution.rs](/src/pbrt_core/bxdf/microfacet_distribution.rs) | Rust | 149 | 11 | 10 | 170 |
| [src/pbrt_core/bxdf/mod.rs](/src/pbrt_core/bxdf/mod.rs) | Rust | 297 | 28 | 11 | 336 |
| [src/pbrt_core/bxdf/pbr.rs](/src/pbrt_core/bxdf/pbr.rs) | Rust | 85 | 1 | 7 | 93 |
| [src/pbrt_core/bxdf/reflection.rs](/src/pbrt_core/bxdf/reflection.rs) | Rust | 135 | 0 | 8 | 143 |
| [src/pbrt_core/bxdf/specular.rs](/src/pbrt_core/bxdf/specular.rs) | Rust | 53 | 7 | 3 | 63 |
| [src/pbrt_core/camera/mod.rs](/src/pbrt_core/camera/mod.rs) | Rust | 69 | 7 | 7 | 83 |
| [src/pbrt_core/filter.rs](/src/pbrt_core/filter.rs) | Rust | 26 | 0 | 0 | 26 |
| [src/pbrt_core/integrator/direct.rs](/src/pbrt_core/integrator/direct.rs) | Rust | 52 | 2 | 5 | 59 |
| [src/pbrt_core/integrator/mod.rs](/src/pbrt_core/integrator/mod.rs) | Rust | 398 | 6 | 22 | 426 |
| [src/pbrt_core/integrator/path.rs](/src/pbrt_core/integrator/path.rs) | Rust | 92 | 5 | 8 | 105 |
| [src/pbrt_core/light/area.rs](/src/pbrt_core/light/area.rs) | Rust | 121 | 0 | 9 | 130 |
| [src/pbrt_core/light/inf.rs](/src/pbrt_core/light/inf.rs) | Rust | 117 | 7 | 13 | 137 |
| [src/pbrt_core/light/mod.rs](/src/pbrt_core/light/mod.rs) | Rust | 199 | 2 | 22 | 223 |
| [src/pbrt_core/light/point.rs](/src/pbrt_core/light/point.rs) | Rust | 77 | 1 | 9 | 87 |
| [src/pbrt_core/light/spot.rs](/src/pbrt_core/light/spot.rs) | Rust | 0 | 0 | 1 | 1 |
| [src/pbrt_core/load/gltfload.rs](/src/pbrt_core/load/gltfload.rs) | Rust | 258 | 14 | 9 | 281 |
| [src/pbrt_core/load/jsonload.rs](/src/pbrt_core/load/jsonload.rs) | Rust | 0 | 0 | 1 | 1 |
| [src/pbrt_core/load/mod.rs](/src/pbrt_core/load/mod.rs) | Rust | 140 | 1 | 8 | 149 |
| [src/pbrt_core/load/myload.rs](/src/pbrt_core/load/myload.rs) | Rust | 374 | 12 | 21 | 407 |
| [src/pbrt_core/load/objload.rs](/src/pbrt_core/load/objload.rs) | Rust | 45 | 0 | 5 | 50 |
| [src/pbrt_core/load/tomlload.rs](/src/pbrt_core/load/tomlload.rs) | Rust | 357 | 6 | 21 | 384 |
| [src/pbrt_core/material/disney.rs](/src/pbrt_core/material/disney.rs) | Rust | 0 | 73 | 6 | 79 |
| [src/pbrt_core/material/matte.rs](/src/pbrt_core/material/matte.rs) | Rust | 44 | 0 | 6 | 50 |
| [src/pbrt_core/material/metal.rs](/src/pbrt_core/material/metal.rs) | Rust | 56 | 7 | 5 | 68 |
| [src/pbrt_core/material/mirror.rs](/src/pbrt_core/material/mirror.rs) | Rust | 23 | 0 | 5 | 28 |
| [src/pbrt_core/material/mod.rs](/src/pbrt_core/material/mod.rs) | Rust | 149 | 10 | 10 | 169 |
| [src/pbrt_core/material/pbr.rs](/src/pbrt_core/material/pbr.rs) | Rust | 79 | 11 | 4 | 94 |
| [src/pbrt_core/material/plastic.rs](/src/pbrt_core/material/plastic.rs) | Rust | 35 | 0 | 5 | 40 |
| [src/pbrt_core/mod.rs](/src/pbrt_core/mod.rs) | Rust | 12 | 8 | 11 | 31 |
| [src/pbrt_core/primitive/bvh.rs](/src/pbrt_core/primitive/bvh.rs) | Rust | 46 | 0 | 8 | 54 |
| [src/pbrt_core/primitive/mesh.rs](/src/pbrt_core/primitive/mesh.rs) | Rust | 41 | 0 | 2 | 43 |
| [src/pbrt_core/primitive/mod.rs](/src/pbrt_core/primitive/mod.rs) | Rust | 191 | 9 | 6 | 206 |
| [src/pbrt_core/primitive/shape/cylinder.rs](/src/pbrt_core/primitive/shape/cylinder.rs) | Rust | 121 | 0 | 9 | 130 |
| [src/pbrt_core/primitive/shape/disk.rs](/src/pbrt_core/primitive/shape/disk.rs) | Rust | 93 | 0 | 12 | 105 |
| [src/pbrt_core/primitive/shape/rectangle.rs](/src/pbrt_core/primitive/shape/rectangle.rs) | Rust | 105 | 1 | 2 | 108 |
| [src/pbrt_core/primitive/shape/shpere.rs](/src/pbrt_core/primitive/shape/shpere.rs) | Rust | 124 | 4 | 5 | 133 |
| [src/pbrt_core/primitive/shape/triangle.rs](/src/pbrt_core/primitive/shape/triangle.rs) | Rust | 171 | 1 | 14 | 186 |
| [src/pbrt_core/sampler/distribution_1d.rs](/src/pbrt_core/sampler/distribution_1d.rs) | Rust | 55 | 1 | 2 | 58 |
| [src/pbrt_core/sampler/mod.rs](/src/pbrt_core/sampler/mod.rs) | Rust | 88 | 0 | 3 | 91 |
| [src/pbrt_core/texture/constant.rs](/src/pbrt_core/texture/constant.rs) | Rust | 21 | 0 | 3 | 24 |
| [src/pbrt_core/texture/image.rs](/src/pbrt_core/texture/image.rs) | Rust | 21 | 0 | 6 | 27 |
| [src/pbrt_core/texture/mix.rs](/src/pbrt_core/texture/mix.rs) | Rust | 0 | 0 | 1 | 1 |
| [src/pbrt_core/texture/mod.rs](/src/pbrt_core/texture/mod.rs) | Rust | 10 | 0 | 2 | 12 |
| [src/pbrt_core/texture/scale.rs](/src/pbrt_core/texture/scale.rs) | Rust | 17 | 0 | 0 | 17 |
| [src/pbrt_core/tool/build.rs](/src/pbrt_core/tool/build.rs) | Rust | 21 | 0 | 4 | 25 |
| [src/pbrt_core/tool/color.rs](/src/pbrt_core/tool/color.rs) | Rust | 141 | 1 | 6 | 148 |
| [src/pbrt_core/tool/error.rs](/src/pbrt_core/tool/error.rs) | Rust | 10 | 0 | 1 | 11 |
| [src/pbrt_core/tool/film.rs](/src/pbrt_core/tool/film.rs) | Rust | 91 | 6 | 6 | 103 |
| [src/pbrt_core/tool/func.rs](/src/pbrt_core/tool/func.rs) | Rust | 169 | 8 | 17 | 194 |
| [src/pbrt_core/tool/log.rs](/src/pbrt_core/tool/log.rs) | Rust | 20 | 0 | 3 | 23 |
| [src/pbrt_core/tool/mipmap.rs](/src/pbrt_core/tool/mipmap.rs) | Rust | 195 | 8 | 5 | 208 |
| [src/pbrt_core/tool/mod.rs](/src/pbrt_core/tool/mod.rs) | Rust | 272 | 9 | 12 | 293 |
| [src/pbrt_core/tool/sence.rs](/src/pbrt_core/tool/sence.rs) | Rust | 105 | 3 | 13 | 121 |
| [src/pbrt_core/tool/setting.rs](/src/pbrt_core/tool/setting.rs) | Rust | 21 | 0 | 5 | 26 |
| [src/pbrt_core/tool/tile.rs](/src/pbrt_core/tool/tile.rs) | Rust | 88 | 3 | 4 | 95 |
| [src/test/mod.rs](/src/test/mod.rs) | Rust | 19 | 1 | 8 | 28 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)