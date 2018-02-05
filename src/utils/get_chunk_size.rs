
///Limits picking chunk size to be no bigger than the remaining characters.
///@param  {Number} max index limit
///@param  {Number} remaining
///@return {Number}

fn get_chunk_size(max = 0, remaining = 0) {
  return Math.min(max, remaining);
}


