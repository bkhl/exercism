local function parse(s)
    local matrix = {}

    local x, y = 1, 1
    for chunk in s:gmatch("%d+%s?") do
        local digits, separator = chunk:match("(%d+)(%s?)")

        if not matrix[y] then matrix[y] = {} end

        matrix[y][x] = tonumber(digits)

        if separator == ' ' then
            x = x + 1
        elseif separator == '\n' then
            x = 1
            y = y + 1
        end
    end

    return matrix
end

local function equip(matrix)
    matrix.row = function(n)
        return matrix[n]
    end

    matrix.column = function(n)
        local column = {}

        for i, row in ipairs(matrix) do
            column[i] = row[n]
        end

        return column
    end
end

return function(s)
    local matrix = parse(s)
    equip(matrix)

    return matrix
end
