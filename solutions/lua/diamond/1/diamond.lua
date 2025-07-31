local space_character <const> = 32
local A_offset <const> = 64

local function row(letter, max_letter, width)
    local characters = {}

    for i = 1, width do
        characters[i] = space_character
    end

    local letter_ascii = letter + A_offset
    characters[max_letter - letter + 1] = letter_ascii
    characters[max_letter + letter - 1] = letter_ascii

    return string.char(table.unpack(characters))
end

return function(which)
    local max_letter = which:byte(1) - A_offset
    local width = max_letter * 2 - 1

    local result = {}

    for i = 1, max_letter do
        result[i] = row(i, max_letter, width)
    end

    for i = 1, max_letter - 1 do
        result[max_letter + i] = result[max_letter - i]
    end

    result[max_letter * 2] = ""

    return table.concat(result, "\n")
end
