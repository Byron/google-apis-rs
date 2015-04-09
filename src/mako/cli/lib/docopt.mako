<%namespace name="util" file="../../lib/util.mako"/>\
<%def name="new(c)">\
docopt!(Args derive Debug, "
Usage: ${util.program_name()} [options] (<OUTPUT-FILE>|-)
       ${util.program_name()} --help

Options:
--width <X>                    The width of the output image [default: 1024]
--height <Y>                   The height of the output image [default: 1024]
--samples-per-pixel <SAMPLES>  Amount of samples per pixel. 4 means 16 over-samples [default: 1]
--num-cores <NUM_CORES>        Amount of cores to do the rendering on [default: 1]
                               If this is not set, you may also use the RTRACEMAXPROCS
                               environment variable, e.g. RTRACEMAXPROCS=4.
                               The commandline always overrides environment variables.

<OUTPUT-FILE>|-     Either a file with .tga extension, or - to write file to stdout
"
, flag_samples_per_pixel: u16
, flag_height: u16
, flag_width: u16
, flag_num_cores: usize);
</%def>