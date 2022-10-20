#!/usr/bin/env python3
import os
import sys
import struct
import hashlib
from elftools.elf.elffile import ELFFile

def load_and_convert(fn):
  print("compiling ", os.path.abspath(fn))

  elf = open(fn, "rb")
  data = elf.read()
  elf.seek(0)

  elffile = ELFFile(elf)

  end_addr = 0
  for seg in elffile.iter_segments():
    end_addr = max(end_addr, seg.header.p_vaddr + seg.header.p_memsz)
  for section in elffile.iter_sections():
    if section.name == ".got":
      end_addr = max(end_addr, section.header.sh_addr + section.header.sh_size)

  # program memory (16 MB)
  prog_size = (end_addr+0xFFF) & ~0xFFF
  prog_dat = bytearray(prog_size)
  print("malloced 0x%x for program" % prog_size)

  for seg in elffile.iter_segments():
    print(seg.header, hex(seg.header.p_vaddr))
    prog_dat[seg.header.p_vaddr:seg.header.p_vaddr+len(seg.data())] = seg.data()

  entry = elffile.header.e_entry
  print("entrypoint: 0x%x" % entry)

  # moved to MIPS
  sf = os.path.join(os.path.dirname(os.path.abspath(__file__)), "startup", "startup.bin")
  start = open(sf, "rb").read() + struct.pack(">I", entry)
  prog_dat[:len(start)] = start
  entry = 0

  
  # Copy the GOT section to its address in the prog_dat blob.
  for section in elffile.iter_sections():
    if section.name == ".got":
      print(hex(section.header.sh_addr))
      prog_dat[section.header.sh_addr:section.header.sh_addr+len(section.data())] = section.data()
      print("copied .got to 0x%x" % (section.header.sh_addr))

  return prog_dat, prog_size

if __name__ == "__main__":
  fn = sys.argv[1]
  outfn = sys.argv[2]

  prog_dat, prog_size = load_and_convert(fn)
  print("compiled %d bytes with md5 %s" % (prog_size, hashlib.md5(prog_dat).hexdigest()))

  with open(outfn, "wb") as f:
    f.write(prog_dat)