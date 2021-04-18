brightman 0.1
D. Scott Boggs

USAGE:
    brightman [FLAGS] [OPTIONS]

FLAGS:
    -h, --help            Prints help information
    -l, --list-devices    
    -V, --version         Prints version information

OPTIONS:
    -d, --decrease-by <decrease-by>    [default: 0]
        --device <device>              [default: error if more than one display present]
    -i, --increase-by <increase-by>    [default: 0]
        --level <level>                [default: 0]

---

i3/sway config options:

bindsym XF86MonBrightnessUp exec brightman --increase-by 5
bindsym XF86MonBrightnessDown exec brightman --decrease-by 5

