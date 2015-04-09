import util

# transform name to be a suitable subcommand
def mangle_subcommand(name):
    return util.camel_to_under(name).replace('_', '-').replace('.', '-')
