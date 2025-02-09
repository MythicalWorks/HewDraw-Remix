#!/usr/bin/python3
import shutil, os, sys, pkgutil, characters

if "help" in sys.argv or "--help" in sys.argv or "-h" in sys.argv:
  print("no arguments required for simple build. To build parts of the project"
    + " as a development reloadable plugin, use argument 'dev=mario,luigi,captain'"
    + " to specify which character crates to build into the reloadable dev plugin.")
  print("For example:")
  print("\t./build.py debug dev=mario,luigi,captain\n")
  print("\t./build.py release dev=captain\n")
  exit(0)

# handle fallback exe on windows
if os.name == 'nt':
  print("windows build!")
  user_profile = os.environ['USERPROFILE']
  
  if not user_profile:
    exit("user profile not found!")
  else:
    print("user profile: " + user_profile)

  fallback = os.path.join(user_profile, '.rustup', 'fallback', 'cargo.exe')
  print("checking for fallback cargo in: " + fallback)
  if os.path.exists(fallback):
    print("fallback found: " + fallback)
    os.remove(fallback)
 
characters = characters.characters

is_dev_build = False
plugin_subpath = "skyline/plugins/"
development_subpath = "smashline/"
ryujinx_rom_path = "mods/contents/01006a800016e000/skyline/romfs"
switch_rom_path = "atmosphere/contents/01006a800016e000/romfs"

current_dir = os.getcwd()
os.chdir('..')

print("arguments: " + ' '.join(sys.argv))

allow_build_dev = True
if "nodev" in sys.argv:
  allow_build_dev = False

release_arg = "--release"
build_type = "release"
is_publish = False
if "release" in sys.argv or "--release" in sys.argv:
  release_arg = "--release"
  build_type = "release"
if "debug" in sys.argv or "--debug" in sys.argv:
  release_arg = ""
  build_type = "debug"
elif "publish" in sys.argv or "--publish" in sys.argv:
  release_arg = "--release"
  build_type = "release"
  is_publish = True

if is_publish:
  allow_dev_build = False

# if staging folder exists, delete it
if "build" in os.listdir('.'):
  shutil.rmtree('build')
os.mkdir('build')


# search for dev plugin args
dev_characters = set()
for arg in sys.argv:
  if "dev" in arg:

    # if theres no equals, break
    if not "=" in arg:
      print("dev specified, but no character arguments given!\n please use 'dev=mario,luigi,samus' format")
      break
    
    # set that this is a development build
    is_dev_build = True

    # get the list of characters
    char_list = (arg.split('=')[1]).split(",")

    # add each character to the set
    for char in char_list:
      if char not in characters:
        print("fighter " + char + " does not exist! (are you using the ingame name for the character?) Valid names are:\n")
        for char_ok in characters:
          print(char_ok)
        exit()
      dev_characters.add(char)

if (is_dev_build and not is_publish):
    
  # special build commands for regular vs development plugins
  dev_args = ""
  if len(dev_characters) > 0:
    # add all of the characters
    dev_args += ' --no-default-features --features="runtime"'
    for character in dev_characters:
      dev_args += ',"' + character + '"'
  else:
    print("ERROR: No character arguments given!")
  
  # build the dev plugin with args
  os.environ["CARGO_TARGET_DIR"] = os.path.join("target", "development")
  
  if allow_build_dev:
    print("release arg: " + release_arg)
    pkgutil.build(release_arg, dev_args)
    print("subpath: " + development_subpath)
    print("type: " + build_type)
    pkgutil.collect_plugin("hdr-switch", os.path.join(switch_rom_path, development_subpath), build_type, "development.nro", "development")
    pkgutil.collect_plugin("hdr-ryujinx", os.path.join(ryujinx_rom_path, development_subpath), build_type, "development.nro", "development")

  # setup normal nro
  non_dev_characters = characters.copy()

  # remove any dev characters
  for char in dev_characters:
    non_dev_characters.remove(char)

  plugin_args = " --no-default-features "
  if len(non_dev_characters) > 0:
    # add each non dev character
    plugin_args += "--features="
    no_comma = True
    for arg in iter(non_dev_characters):
      if no_comma:
        plugin_args += '"' + arg + '"'
        no_comma = False
      else:
        plugin_args += ',"' + arg + '"'

  if not "dev-only" in sys.argv:
    # build the regular plugin with args
    os.environ["CARGO_TARGET_DIR"] = os.path.join("target", "standalone")
    pkgutil.build(release_arg, plugin_args)

  # collect switch plugin
  pkgutil.collect_plugin("hdr-switch", 
    os.path.join(switch_rom_path, plugin_subpath), 
    build_type, "libhdr.nro", "standalone")

    # collect switch romfs
  pkgutil.collect_romfs("hdr-switch", "")

  # collect ryujinx plugin
  pkgutil.collect_plugin("hdr-ryujinx", 
    os.path.join(ryujinx_rom_path, plugin_subpath), 
    build_type, "libhdr.nro", "standalone")
  
  # collect ryujinx romfs
  pkgutil.collect_romfs("hdr-ryujinx", "sdcard")


else:
  # simple build
  if is_publish:
    pkgutil.build(release_arg, "--features updater")
  else:
    pkgutil.build(release_arg, "")

  # collect switch package
  pkgutil.collect_plugin("hdr-switch", 
    os.path.join(switch_rom_path, plugin_subpath), 
    build_type, "libhdr.nro")

  # collect switch romfs
  pkgutil.collect_romfs("hdr-switch", "")


  # collect ryujinx plugin
  pkgutil.collect_plugin("hdr-ryujinx", 
    os.path.join(ryujinx_rom_path, plugin_subpath), 
    build_type, "libhdr.nro")
  
  # collect ryujinx romfs
  pkgutil.collect_romfs("hdr-ryujinx", "sdcard")

os.chdir(current_dir)


