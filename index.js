var ffi = require('ffi');
var path = require('path');

var lib = ffi.Library(path.join(__dirname, './rust-dylib/libfind_files'), {
  find_files: ['char *', ['string']],
  free_memory: ['void', ['char *']]
});

function find(s) {
  var json_string = lib.find_files(s);

  let save_json = JSON.parse(json_string.readCString());

  lib.free_memory(json_string);
  return save_json;
}

module.exports = {
  find
};
