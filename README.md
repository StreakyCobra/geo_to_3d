# Create 3D models from geographic data

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-generate-toc again -->
**Table of Contents**

- [Create 3D models from geographic data](#create-3d-models-from-geographic-data)
    - [Presentation](#presentation)
    - [Requirements](#requirements)
        - [At build time](#at-build-time)
        - [At run time](#at-run-time)
    - [How to build](#how-to-build)
    - [License](#license)
        - [Software](#software)
        - [Data](#data)

<!-- markdown-toc end -->

## Presentation

`geo_to_3d` is a tool to create 3D models from geographic data.

The Digital Elevation Models (DEM) used to generate the 3D models are a courtesy
of *Jonathan de Ferranti* at <http://www.viewfinderpanoramas.org>.

## Requirements

### At build time

-   [rust](https://www.rust-lang.org/) (for compilation)
-   [cargo](https://crates.io/) (for dependencies management)

### At run time

-   [wget](https://www.gnu.org/software/wget/wget.html) (to download geographic
    data)
-   [unzip](http://www.info-zip.org/UnZip.html) (to extract geographic data)

## How to build

In order to build this project, you will need to install tooling for
compiling [rust](https://www.rust-lang.org/) code. See
the [requirements](#requirements) section for the complete list.

The project can be build with:

```shell
$ cargo build
```

This will generate a `target/debug/geo_to_3d` executable that can be run
with the following command:

```shell
$ ./target/debug/geo_to_3d -h
```

Alternatively, you can directly build and run the project with:

```shell
$ cargo run -q -- -h
```

## License

### Software

This software is licensed under the term of
the [GPL v3.0](https://www.gnu.org/licenses/gpl-3.0.html) license:

> geo\_to\_3d is a tool to create 3D models from geographic data.
> Copyright (C) 2016-2017 Fabien Dubosson &lt;fabien.dubosson@gmail.com&gt;
>
> This program is free software: you can redistribute it and/or modify
> it under the terms of the GNU General Public License as published by
> the Free Software Foundation, either version 3 of the License, or (at
> your option) any later version.
>
> This program is distributed in the hope that it will be useful, but
> WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
> General Public License for more details.
>
> You should have received a copy of the GNU General Public License
> along with this program. If not, see <http://www.gnu.org/licenses/>.

See [LICENSE.md](LICENSE.md) for the complete license.

### Data

This software is downloading topographic data
from [viewfinderpanoramas](http://www.viewfinderpanoramas.org/), a project of
*Jonathan de Ferranti*. The use of these data are conditioned to
its [Term of Use](http://www.viewfinderpanoramas.org/).
