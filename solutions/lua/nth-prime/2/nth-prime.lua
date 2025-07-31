function is_prime(n)
    for i = 2, math.sqrt(n) do
        if n % i == 0 then
            return false
        end
    end
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
