local DELIMITERS = {
  ["("] = ")",
  ["["] = "]",
  ["{"] = "}",
}

local R_DELIMETERS = {}
for _, r_delimiter in pairs(DELIMITERS) do
  R_DELIMETERS[r_delimiter] = true
end

--- Check if a string has balanced delimiters.
---@param s string
---@return boolean
local function valid(s)
  local next_r_delimeter = nil ---@type string?
  local r_delimiters_stack = {} ---@type string[]

  for char in s:gmatch "." do
    if char == next_r_delimeter then
      next_r_delimeter = table.remove(r_delimiters_stack)
    elseif R_DELIMETERS[char] then
      return false
    else
      local new_r_delimeter = DELIMITERS[char]
      if new_r_delimeter then
        table.insert(r_delimiters_stack, next_r_delimeter)
        next_r_delimeter = new_r_delimeter
      end
    end
  end

  return next_r_delimeter == nil
end

return {
  valid = valid,
}
