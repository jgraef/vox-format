use crate::vox::{
    Color,
    Palette,
};

pub const DEFAULT_PALETTE: Palette = Palette {
    colors: [
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }, // #000000
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }, // #ffffffff
        Color {
            r: 255,
            g: 255,
            b: 204,
            a: 255,
        }, // #ffccffff
        Color {
            r: 255,
            g: 255,
            b: 153,
            a: 255,
        }, // #ff99ffff
        Color {
            r: 255,
            g: 255,
            b: 102,
            a: 255,
        }, // #ff66ffff
        Color {
            r: 255,
            g: 255,
            b: 51,
            a: 255,
        }, // #ff33ffff
        Color {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        }, // #ff00ffff
        Color {
            r: 255,
            g: 204,
            b: 255,
            a: 255,
        }, // #ffffccff
        Color {
            r: 255,
            g: 204,
            b: 204,
            a: 255,
        }, // #ffccccff
        Color {
            r: 255,
            g: 204,
            b: 153,
            a: 255,
        }, // #ff99ccff
        Color {
            r: 255,
            g: 204,
            b: 102,
            a: 255,
        }, // #ff66ccff
        Color {
            r: 255,
            g: 204,
            b: 51,
            a: 255,
        }, // #ff33ccff
        Color {
            r: 255,
            g: 204,
            b: 0,
            a: 255,
        }, // #ff00ccff
        Color {
            r: 255,
            g: 153,
            b: 255,
            a: 255,
        }, // #ffff99ff
        Color {
            r: 255,
            g: 153,
            b: 204,
            a: 255,
        }, // #ffcc99ff
        Color {
            r: 255,
            g: 153,
            b: 153,
            a: 255,
        }, // #ff9999ff
        Color {
            r: 255,
            g: 153,
            b: 102,
            a: 255,
        }, // #ff6699ff
        Color {
            r: 255,
            g: 153,
            b: 51,
            a: 255,
        }, // #ff3399ff
        Color {
            r: 255,
            g: 153,
            b: 0,
            a: 255,
        }, // #ff0099ff
        Color {
            r: 255,
            g: 102,
            b: 255,
            a: 255,
        }, // #ffff66ff
        Color {
            r: 255,
            g: 102,
            b: 204,
            a: 255,
        }, // #ffcc66ff
        Color {
            r: 255,
            g: 102,
            b: 153,
            a: 255,
        }, // #ff9966ff
        Color {
            r: 255,
            g: 102,
            b: 102,
            a: 255,
        }, // #ff6666ff
        Color {
            r: 255,
            g: 102,
            b: 51,
            a: 255,
        }, // #ff3366ff
        Color {
            r: 255,
            g: 102,
            b: 0,
            a: 255,
        }, // #ff0066ff
        Color {
            r: 255,
            g: 51,
            b: 255,
            a: 255,
        }, // #ffff33ff
        Color {
            r: 255,
            g: 51,
            b: 204,
            a: 255,
        }, // #ffcc33ff
        Color {
            r: 255,
            g: 51,
            b: 153,
            a: 255,
        }, // #ff9933ff
        Color {
            r: 255,
            g: 51,
            b: 102,
            a: 255,
        }, // #ff6633ff
        Color {
            r: 255,
            g: 51,
            b: 51,
            a: 255,
        }, // #ff3333ff
        Color {
            r: 255,
            g: 51,
            b: 0,
            a: 255,
        }, // #ff0033ff
        Color {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }, // #ffff00ff
        Color {
            r: 255,
            g: 0,
            b: 204,
            a: 255,
        }, // #ffcc00ff
        Color {
            r: 255,
            g: 0,
            b: 153,
            a: 255,
        }, // #ff9900ff
        Color {
            r: 255,
            g: 0,
            b: 102,
            a: 255,
        }, // #ff6600ff
        Color {
            r: 255,
            g: 0,
            b: 51,
            a: 255,
        }, // #ff3300ff
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff0000ff
        Color {
            r: 204,
            g: 255,
            b: 255,
            a: 255,
        }, // #ffffffcc
        Color {
            r: 204,
            g: 255,
            b: 204,
            a: 255,
        }, // #ffccffcc
        Color {
            r: 204,
            g: 255,
            b: 153,
            a: 255,
        }, // #ff99ffcc
        Color {
            r: 204,
            g: 255,
            b: 102,
            a: 255,
        }, // #ff66ffcc
        Color {
            r: 204,
            g: 255,
            b: 51,
            a: 255,
        }, // #ff33ffcc
        Color {
            r: 204,
            g: 255,
            b: 0,
            a: 255,
        }, // #ff00ffcc
        Color {
            r: 204,
            g: 204,
            b: 255,
            a: 255,
        }, // #ffffcccc
        Color {
            r: 204,
            g: 204,
            b: 204,
            a: 255,
        }, // #ffcccccc
        Color {
            r: 204,
            g: 204,
            b: 153,
            a: 255,
        }, // #ff99cccc
        Color {
            r: 204,
            g: 204,
            b: 102,
            a: 255,
        }, // #ff66cccc
        Color {
            r: 204,
            g: 204,
            b: 51,
            a: 255,
        }, // #ff33cccc
        Color {
            r: 204,
            g: 204,
            b: 0,
            a: 255,
        }, // #ff00cccc
        Color {
            r: 204,
            g: 153,
            b: 255,
            a: 255,
        }, // #ffff99cc
        Color {
            r: 204,
            g: 153,
            b: 204,
            a: 255,
        }, // #ffcc99cc
        Color {
            r: 204,
            g: 153,
            b: 153,
            a: 255,
        }, // #ff9999cc
        Color {
            r: 204,
            g: 153,
            b: 102,
            a: 255,
        }, // #ff6699cc
        Color {
            r: 204,
            g: 153,
            b: 51,
            a: 255,
        }, // #ff3399cc
        Color {
            r: 204,
            g: 153,
            b: 0,
            a: 255,
        }, // #ff0099cc
        Color {
            r: 204,
            g: 102,
            b: 255,
            a: 255,
        }, // #ffff66cc
        Color {
            r: 204,
            g: 102,
            b: 204,
            a: 255,
        }, // #ffcc66cc
        Color {
            r: 204,
            g: 102,
            b: 153,
            a: 255,
        }, // #ff9966cc
        Color {
            r: 204,
            g: 102,
            b: 102,
            a: 255,
        }, // #ff6666cc
        Color {
            r: 204,
            g: 102,
            b: 51,
            a: 255,
        }, // #ff3366cc
        Color {
            r: 204,
            g: 102,
            b: 0,
            a: 255,
        }, // #ff0066cc
        Color {
            r: 204,
            g: 51,
            b: 255,
            a: 255,
        }, // #ffff33cc
        Color {
            r: 204,
            g: 51,
            b: 204,
            a: 255,
        }, // #ffcc33cc
        Color {
            r: 204,
            g: 51,
            b: 153,
            a: 255,
        }, // #ff9933cc
        Color {
            r: 204,
            g: 51,
            b: 102,
            a: 255,
        }, // #ff6633cc
        Color {
            r: 204,
            g: 51,
            b: 51,
            a: 255,
        }, // #ff3333cc
        Color {
            r: 204,
            g: 51,
            b: 0,
            a: 255,
        }, // #ff0033cc
        Color {
            r: 204,
            g: 0,
            b: 255,
            a: 255,
        }, // #ffff00cc
        Color {
            r: 204,
            g: 0,
            b: 204,
            a: 255,
        }, // #ffcc00cc
        Color {
            r: 204,
            g: 0,
            b: 153,
            a: 255,
        }, // #ff9900cc
        Color {
            r: 204,
            g: 0,
            b: 102,
            a: 255,
        }, // #ff6600cc
        Color {
            r: 204,
            g: 0,
            b: 51,
            a: 255,
        }, // #ff3300cc
        Color {
            r: 204,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff0000cc
        Color {
            r: 153,
            g: 255,
            b: 255,
            a: 255,
        }, // #ffffff99
        Color {
            r: 153,
            g: 255,
            b: 204,
            a: 255,
        }, // #ffccff99
        Color {
            r: 153,
            g: 255,
            b: 153,
            a: 255,
        }, // #ff99ff99
        Color {
            r: 153,
            g: 255,
            b: 102,
            a: 255,
        }, // #ff66ff99
        Color {
            r: 153,
            g: 255,
            b: 51,
            a: 255,
        }, // #ff33ff99
        Color {
            r: 153,
            g: 255,
            b: 0,
            a: 255,
        }, // #ff00ff99
        Color {
            r: 153,
            g: 204,
            b: 255,
            a: 255,
        }, // #ffffcc99
        Color {
            r: 153,
            g: 204,
            b: 204,
            a: 255,
        }, // #ffcccc99
        Color {
            r: 153,
            g: 204,
            b: 153,
            a: 255,
        }, // #ff99cc99
        Color {
            r: 153,
            g: 204,
            b: 102,
            a: 255,
        }, // #ff66cc99
        Color {
            r: 153,
            g: 204,
            b: 51,
            a: 255,
        }, // #ff33cc99
        Color {
            r: 153,
            g: 204,
            b: 0,
            a: 255,
        }, // #ff00cc99
        Color {
            r: 153,
            g: 153,
            b: 255,
            a: 255,
        }, // #ffff9999
        Color {
            r: 153,
            g: 153,
            b: 204,
            a: 255,
        }, // #ffcc9999
        Color {
            r: 153,
            g: 153,
            b: 153,
            a: 255,
        }, // #ff999999
        Color {
            r: 153,
            g: 153,
            b: 102,
            a: 255,
        }, // #ff669999
        Color {
            r: 153,
            g: 153,
            b: 51,
            a: 255,
        }, // #ff339999
        Color {
            r: 153,
            g: 153,
            b: 0,
            a: 255,
        }, // #ff009999
        Color {
            r: 153,
            g: 102,
            b: 255,
            a: 255,
        }, // #ffff6699
        Color {
            r: 153,
            g: 102,
            b: 204,
            a: 255,
        }, // #ffcc6699
        Color {
            r: 153,
            g: 102,
            b: 153,
            a: 255,
        }, // #ff996699
        Color {
            r: 153,
            g: 102,
            b: 102,
            a: 255,
        }, // #ff666699
        Color {
            r: 153,
            g: 102,
            b: 51,
            a: 255,
        }, // #ff336699
        Color {
            r: 153,
            g: 102,
            b: 0,
            a: 255,
        }, // #ff006699
        Color {
            r: 153,
            g: 51,
            b: 255,
            a: 255,
        }, // #ffff3399
        Color {
            r: 153,
            g: 51,
            b: 204,
            a: 255,
        }, // #ffcc3399
        Color {
            r: 153,
            g: 51,
            b: 153,
            a: 255,
        }, // #ff993399
        Color {
            r: 153,
            g: 51,
            b: 102,
            a: 255,
        }, // #ff663399
        Color {
            r: 153,
            g: 51,
            b: 51,
            a: 255,
        }, // #ff333399
        Color {
            r: 153,
            g: 51,
            b: 0,
            a: 255,
        }, // #ff003399
        Color {
            r: 153,
            g: 0,
            b: 255,
            a: 255,
        }, // #ffff0099
        Color {
            r: 153,
            g: 0,
            b: 204,
            a: 255,
        }, // #ffcc0099
        Color {
            r: 153,
            g: 0,
            b: 153,
            a: 255,
        }, // #ff990099
        Color {
            r: 153,
            g: 0,
            b: 102,
            a: 255,
        }, // #ff660099
        Color {
            r: 153,
            g: 0,
            b: 51,
            a: 255,
        }, // #ff330099
        Color {
            r: 153,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000099
        Color {
            r: 102,
            g: 255,
            b: 255,
            a: 255,
        }, // #ffffff66
        Color {
            r: 102,
            g: 255,
            b: 204,
            a: 255,
        }, // #ffccff66
        Color {
            r: 102,
            g: 255,
            b: 153,
            a: 255,
        }, // #ff99ff66
        Color {
            r: 102,
            g: 255,
            b: 102,
            a: 255,
        }, // #ff66ff66
        Color {
            r: 102,
            g: 255,
            b: 51,
            a: 255,
        }, // #ff33ff66
        Color {
            r: 102,
            g: 255,
            b: 0,
            a: 255,
        }, // #ff00ff66
        Color {
            r: 102,
            g: 204,
            b: 255,
            a: 255,
        }, // #ffffcc66
        Color {
            r: 102,
            g: 204,
            b: 204,
            a: 255,
        }, // #ffcccc66
        Color {
            r: 102,
            g: 204,
            b: 153,
            a: 255,
        }, // #ff99cc66
        Color {
            r: 102,
            g: 204,
            b: 102,
            a: 255,
        }, // #ff66cc66
        Color {
            r: 102,
            g: 204,
            b: 51,
            a: 255,
        }, // #ff33cc66
        Color {
            r: 102,
            g: 204,
            b: 0,
            a: 255,
        }, // #ff00cc66
        Color {
            r: 102,
            g: 153,
            b: 255,
            a: 255,
        }, // #ffff9966
        Color {
            r: 102,
            g: 153,
            b: 204,
            a: 255,
        }, // #ffcc9966
        Color {
            r: 102,
            g: 153,
            b: 153,
            a: 255,
        }, // #ff999966
        Color {
            r: 102,
            g: 153,
            b: 102,
            a: 255,
        }, // #ff669966
        Color {
            r: 102,
            g: 153,
            b: 51,
            a: 255,
        }, // #ff339966
        Color {
            r: 102,
            g: 153,
            b: 0,
            a: 255,
        }, // #ff009966
        Color {
            r: 102,
            g: 102,
            b: 255,
            a: 255,
        }, // #ffff6666
        Color {
            r: 102,
            g: 102,
            b: 204,
            a: 255,
        }, // #ffcc6666
        Color {
            r: 102,
            g: 102,
            b: 153,
            a: 255,
        }, // #ff996666
        Color {
            r: 102,
            g: 102,
            b: 102,
            a: 255,
        }, // #ff666666
        Color {
            r: 102,
            g: 102,
            b: 51,
            a: 255,
        }, // #ff336666
        Color {
            r: 102,
            g: 102,
            b: 0,
            a: 255,
        }, // #ff006666
        Color {
            r: 102,
            g: 51,
            b: 255,
            a: 255,
        }, // #ffff3366
        Color {
            r: 102,
            g: 51,
            b: 204,
            a: 255,
        }, // #ffcc3366
        Color {
            r: 102,
            g: 51,
            b: 153,
            a: 255,
        }, // #ff993366
        Color {
            r: 102,
            g: 51,
            b: 102,
            a: 255,
        }, // #ff663366
        Color {
            r: 102,
            g: 51,
            b: 51,
            a: 255,
        }, // #ff333366
        Color {
            r: 102,
            g: 51,
            b: 0,
            a: 255,
        }, // #ff003366
        Color {
            r: 102,
            g: 0,
            b: 255,
            a: 255,
        }, // #ffff0066
        Color {
            r: 102,
            g: 0,
            b: 204,
            a: 255,
        }, // #ffcc0066
        Color {
            r: 102,
            g: 0,
            b: 153,
            a: 255,
        }, // #ff990066
        Color {
            r: 102,
            g: 0,
            b: 102,
            a: 255,
        }, // #ff660066
        Color {
            r: 102,
            g: 0,
            b: 51,
            a: 255,
        }, // #ff330066
        Color {
            r: 102,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000066
        Color {
            r: 51,
            g: 255,
            b: 255,
            a: 255,
        }, // #ffffff33
        Color {
            r: 51,
            g: 255,
            b: 204,
            a: 255,
        }, // #ffccff33
        Color {
            r: 51,
            g: 255,
            b: 153,
            a: 255,
        }, // #ff99ff33
        Color {
            r: 51,
            g: 255,
            b: 102,
            a: 255,
        }, // #ff66ff33
        Color {
            r: 51,
            g: 255,
            b: 51,
            a: 255,
        }, // #ff33ff33
        Color {
            r: 51,
            g: 255,
            b: 0,
            a: 255,
        }, // #ff00ff33
        Color {
            r: 51,
            g: 204,
            b: 255,
            a: 255,
        }, // #ffffcc33
        Color {
            r: 51,
            g: 204,
            b: 204,
            a: 255,
        }, // #ffcccc33
        Color {
            r: 51,
            g: 204,
            b: 153,
            a: 255,
        }, // #ff99cc33
        Color {
            r: 51,
            g: 204,
            b: 102,
            a: 255,
        }, // #ff66cc33
        Color {
            r: 51,
            g: 204,
            b: 51,
            a: 255,
        }, // #ff33cc33
        Color {
            r: 51,
            g: 204,
            b: 0,
            a: 255,
        }, // #ff00cc33
        Color {
            r: 51,
            g: 153,
            b: 255,
            a: 255,
        }, // #ffff9933
        Color {
            r: 51,
            g: 153,
            b: 204,
            a: 255,
        }, // #ffcc9933
        Color {
            r: 51,
            g: 153,
            b: 153,
            a: 255,
        }, // #ff999933
        Color {
            r: 51,
            g: 153,
            b: 102,
            a: 255,
        }, // #ff669933
        Color {
            r: 51,
            g: 153,
            b: 51,
            a: 255,
        }, // #ff339933
        Color {
            r: 51,
            g: 153,
            b: 0,
            a: 255,
        }, // #ff009933
        Color {
            r: 51,
            g: 102,
            b: 255,
            a: 255,
        }, // #ffff6633
        Color {
            r: 51,
            g: 102,
            b: 204,
            a: 255,
        }, // #ffcc6633
        Color {
            r: 51,
            g: 102,
            b: 153,
            a: 255,
        }, // #ff996633
        Color {
            r: 51,
            g: 102,
            b: 102,
            a: 255,
        }, // #ff666633
        Color {
            r: 51,
            g: 102,
            b: 51,
            a: 255,
        }, // #ff336633
        Color {
            r: 51,
            g: 102,
            b: 0,
            a: 255,
        }, // #ff006633
        Color {
            r: 51,
            g: 51,
            b: 255,
            a: 255,
        }, // #ffff3333
        Color {
            r: 51,
            g: 51,
            b: 204,
            a: 255,
        }, // #ffcc3333
        Color {
            r: 51,
            g: 51,
            b: 153,
            a: 255,
        }, // #ff993333
        Color {
            r: 51,
            g: 51,
            b: 102,
            a: 255,
        }, // #ff663333
        Color {
            r: 51,
            g: 51,
            b: 51,
            a: 255,
        }, // #ff333333
        Color {
            r: 51,
            g: 51,
            b: 0,
            a: 255,
        }, // #ff003333
        Color {
            r: 51,
            g: 0,
            b: 255,
            a: 255,
        }, // #ffff0033
        Color {
            r: 51,
            g: 0,
            b: 204,
            a: 255,
        }, // #ffcc0033
        Color {
            r: 51,
            g: 0,
            b: 153,
            a: 255,
        }, // #ff990033
        Color {
            r: 51,
            g: 0,
            b: 102,
            a: 255,
        }, // #ff660033
        Color {
            r: 51,
            g: 0,
            b: 51,
            a: 255,
        }, // #ff330033
        Color {
            r: 51,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000033
        Color {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        }, // #ffffff00
        Color {
            r: 0,
            g: 255,
            b: 204,
            a: 255,
        }, // #ffccff00
        Color {
            r: 0,
            g: 255,
            b: 153,
            a: 255,
        }, // #ff99ff00
        Color {
            r: 0,
            g: 255,
            b: 102,
            a: 255,
        }, // #ff66ff00
        Color {
            r: 0,
            g: 255,
            b: 51,
            a: 255,
        }, // #ff33ff00
        Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }, // #ff00ff00
        Color {
            r: 0,
            g: 204,
            b: 255,
            a: 255,
        }, // #ffffcc00
        Color {
            r: 0,
            g: 204,
            b: 204,
            a: 255,
        }, // #ffcccc00
        Color {
            r: 0,
            g: 204,
            b: 153,
            a: 255,
        }, // #ff99cc00
        Color {
            r: 0,
            g: 204,
            b: 102,
            a: 255,
        }, // #ff66cc00
        Color {
            r: 0,
            g: 204,
            b: 51,
            a: 255,
        }, // #ff33cc00
        Color {
            r: 0,
            g: 204,
            b: 0,
            a: 255,
        }, // #ff00cc00
        Color {
            r: 0,
            g: 153,
            b: 255,
            a: 255,
        }, // #ffff9900
        Color {
            r: 0,
            g: 153,
            b: 204,
            a: 255,
        }, // #ffcc9900
        Color {
            r: 0,
            g: 153,
            b: 153,
            a: 255,
        }, // #ff999900
        Color {
            r: 0,
            g: 153,
            b: 102,
            a: 255,
        }, // #ff669900
        Color {
            r: 0,
            g: 153,
            b: 51,
            a: 255,
        }, // #ff339900
        Color {
            r: 0,
            g: 153,
            b: 0,
            a: 255,
        }, // #ff009900
        Color {
            r: 0,
            g: 102,
            b: 255,
            a: 255,
        }, // #ffff6600
        Color {
            r: 0,
            g: 102,
            b: 204,
            a: 255,
        }, // #ffcc6600
        Color {
            r: 0,
            g: 102,
            b: 153,
            a: 255,
        }, // #ff996600
        Color {
            r: 0,
            g: 102,
            b: 102,
            a: 255,
        }, // #ff666600
        Color {
            r: 0,
            g: 102,
            b: 51,
            a: 255,
        }, // #ff336600
        Color {
            r: 0,
            g: 102,
            b: 0,
            a: 255,
        }, // #ff006600
        Color {
            r: 0,
            g: 51,
            b: 255,
            a: 255,
        }, // #ffff3300
        Color {
            r: 0,
            g: 51,
            b: 204,
            a: 255,
        }, // #ffcc3300
        Color {
            r: 0,
            g: 51,
            b: 153,
            a: 255,
        }, // #ff993300
        Color {
            r: 0,
            g: 51,
            b: 102,
            a: 255,
        }, // #ff663300
        Color {
            r: 0,
            g: 51,
            b: 51,
            a: 255,
        }, // #ff333300
        Color {
            r: 0,
            g: 51,
            b: 0,
            a: 255,
        }, // #ff003300
        Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }, // #ffff0000
        Color {
            r: 0,
            g: 0,
            b: 204,
            a: 255,
        }, // #ffcc0000
        Color {
            r: 0,
            g: 0,
            b: 153,
            a: 255,
        }, // #ff990000
        Color {
            r: 0,
            g: 0,
            b: 102,
            a: 255,
        }, // #ff660000
        Color {
            r: 0,
            g: 0,
            b: 51,
            a: 255,
        }, // #ff330000
        Color {
            r: 238,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff0000ee
        Color {
            r: 221,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff0000dd
        Color {
            r: 187,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff0000bb
        Color {
            r: 170,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff0000aa
        Color {
            r: 136,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000088
        Color {
            r: 119,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000077
        Color {
            r: 85,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000055
        Color {
            r: 68,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000044
        Color {
            r: 34,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000022
        Color {
            r: 17,
            g: 0,
            b: 0,
            a: 255,
        }, // #ff000011
        Color {
            r: 0,
            g: 238,
            b: 0,
            a: 255,
        }, // #ff00ee00
        Color {
            r: 0,
            g: 221,
            b: 0,
            a: 255,
        }, // #ff00dd00
        Color {
            r: 0,
            g: 187,
            b: 0,
            a: 255,
        }, // #ff00bb00
        Color {
            r: 0,
            g: 170,
            b: 0,
            a: 255,
        }, // #ff00aa00
        Color {
            r: 0,
            g: 136,
            b: 0,
            a: 255,
        }, // #ff008800
        Color {
            r: 0,
            g: 119,
            b: 0,
            a: 255,
        }, // #ff007700
        Color {
            r: 0,
            g: 85,
            b: 0,
            a: 255,
        }, // #ff005500
        Color {
            r: 0,
            g: 68,
            b: 0,
            a: 255,
        }, // #ff004400
        Color {
            r: 0,
            g: 34,
            b: 0,
            a: 255,
        }, // #ff002200
        Color {
            r: 0,
            g: 17,
            b: 0,
            a: 255,
        }, // #ff001100
        Color {
            r: 0,
            g: 0,
            b: 238,
            a: 255,
        }, // #ffee0000
        Color {
            r: 0,
            g: 0,
            b: 221,
            a: 255,
        }, // #ffdd0000
        Color {
            r: 0,
            g: 0,
            b: 187,
            a: 255,
        }, // #ffbb0000
        Color {
            r: 0,
            g: 0,
            b: 170,
            a: 255,
        }, // #ffaa0000
        Color {
            r: 0,
            g: 0,
            b: 136,
            a: 255,
        }, // #ff880000
        Color {
            r: 0,
            g: 0,
            b: 119,
            a: 255,
        }, // #ff770000
        Color {
            r: 0,
            g: 0,
            b: 85,
            a: 255,
        }, // #ff550000
        Color {
            r: 0,
            g: 0,
            b: 68,
            a: 255,
        }, // #ff440000
        Color {
            r: 0,
            g: 0,
            b: 34,
            a: 255,
        }, // #ff220000
        Color {
            r: 0,
            g: 0,
            b: 17,
            a: 255,
        }, // #ff110000
        Color {
            r: 238,
            g: 238,
            b: 238,
            a: 255,
        }, // #ffeeeeee
        Color {
            r: 221,
            g: 221,
            b: 221,
            a: 255,
        }, // #ffdddddd
        Color {
            r: 187,
            g: 187,
            b: 187,
            a: 255,
        }, // #ffbbbbbb
        Color {
            r: 170,
            g: 170,
            b: 170,
            a: 255,
        }, // #ffaaaaaa
        Color {
            r: 136,
            g: 136,
            b: 136,
            a: 255,
        }, // #ff888888
        Color {
            r: 119,
            g: 119,
            b: 119,
            a: 255,
        }, // #ff777777
        Color {
            r: 85,
            g: 85,
            b: 85,
            a: 255,
        }, // #ff555555
        Color {
            r: 68,
            g: 68,
            b: 68,
            a: 255,
        }, // #ff444444
        Color {
            r: 34,
            g: 34,
            b: 34,
            a: 255,
        }, // #ff222222
        Color {
            r: 17,
            g: 17,
            b: 17,
            a: 255,
        }, // #ff111111
    ],
};
