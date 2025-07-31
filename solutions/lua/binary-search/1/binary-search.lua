return function(array, target)
    local a = 1
    local b = #array

    while a <= b do
        local i = (a + b) // 2
        local v = array[i]

        if v < target then
            a = i + 1
        elseif target < v then
            b = i - 1
        else
            return i
        end
    end

    return -1
end
