local cache = {}

function is_prime(n)
    if cache[n] ~= nil then
        return cache[n]
    end

    for i = 2, n - 1 do
        if n % i == 0 then
            cache[n] = false
            return false
        end
    end

    cache[n] = true
    return true
end

return function(n)
    if n < 1 then
        error("argument must be a positive integer")
    end

    local i = 2
    while n > 1 do
        i = i + 1
        if is_prime(i) then
            n = n - 1
        end
    end
    return i
end
