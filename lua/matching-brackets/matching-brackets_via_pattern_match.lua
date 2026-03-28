local function has_adjacent_matches(s)
  local r = s:gsub("%(%)", ""):gsub("%[%]", ""):gsub("%{%}", "")
  return r == "" or r ~= s and has_adjacent_matches(r)
end

return {
  valid = function(s) return has_adjacent_matches(s:gsub("[^()[%]{}]", "")) end,
}
