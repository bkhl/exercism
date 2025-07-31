local BankAccount = {}

function BankAccount:new()
    o = { _active = true, _balance = 0 }
    setmetatable(o, self)
    self.__index = self
    return o
end

function BankAccount:balance()
    return self._balance
end

function BankAccount:deposit(n)
    if n <= 0 then error("deposit must be positive") end
    if not self._active then error("cannot deposit to closed account") end
    self._balance = self._balance + n
end

function BankAccount:withdraw(n)
    if n <= 0 then error("withdrawal must be positive") end
    if not self._active then error("cannot withdraw from closed account") end
    if self._balance < n then error("withdrawal must not overdraw") end
    self._balance = self._balance - n
end

function BankAccount:close()
    self._active = false
end

return BankAccount
