return function(s)
    local letters = {}

    for c in s:lower():gmatch"%w" do
        if letters[c] then
            return false
        end

        letters[c] = true
    end

    return true
end
