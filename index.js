var ffi = require('ffi');
var path = require('path');

var lib = ffi.Library(path.join(__dirname, './rust-dylib/libfind_files'), {
  find_files: ['char *', []],
  free_memory: ['void', ['char *']]
});

var json_string = lib.find_files();

var save_json = JSON.parse(json_string.readCString());

lib.free_memory(json_string);

module.exports = {
  mds: save_json
};
