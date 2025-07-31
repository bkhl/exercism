local house = {}

local things = {
    { nil, "horse and the hound and the horn" },
    { "belonged to", "farmer sowing his corn" },
    { "kept", "rooster that crowed in the morn" },
    { "woke", "priest all shaven and shorn" },
    { "married", "man all tattered and torn" },
    { "kissed", "maiden all forlorn" },
    { "milked", "cow with the crumpled horn" },
    { "tossed", "dog" },
    { "worried", "cat" },
    { "killed", "rat" },
    { "ate", "malt" },
    { "lay in", "house that Jack built." },
}

house.verse = function(which)
    local lines = { string.format("This is the %s", things[13 - which][2]) }

    for i = 2, which do
        local thing = things[12 - which + i]
        lines[i] = string.format("that %s the %s", thing[1], thing[2])
    end

    return table.concat(lines, "\n")
end

house.recite = function()
    local verses = {}

    for i = 1, 12 do
        verses[i] = house.verse(i)
    end

    return table.concat(verses, "\n")
end

return house
