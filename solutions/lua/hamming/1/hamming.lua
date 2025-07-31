local Hamming = {}

function Hamming.compute(a,b)
    if string.len(a) ~= string.len(b) then
        return -1
    end

    local result = 0

    for i = 1, string.len(a) do
        if (a:byte(i) ~= b:byte(i)) then
            result = result + 1
        end
    end

    return result
end

return Hamming
