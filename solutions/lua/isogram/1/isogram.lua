return function(s)
    local letters = {}

    for c in s:lower():gmatch"%w" do
        if letters[c] then
            return false
        else
            letters[c] = true
        end
    end

    return true
end
