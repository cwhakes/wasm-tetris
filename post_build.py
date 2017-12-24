from shutil import copyfile
from subprocess import call

copyfile('target/wasm32-unknown-unknown/release/tetris.wasm', 'html/tetris.wasm')
call(['wasm-gc', 'html/tetris.wasm', 'html/program.wasm'])
