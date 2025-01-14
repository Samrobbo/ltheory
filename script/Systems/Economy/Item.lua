local all = {}

local Item = class(function(self, name, mass, energyDensity, distribution)
    self.name = name
    self.mass = mass or 1
    self.energy = Math.Round(math.max(1, (energyDensity or 1) * self.mass))
    self.distribution = distribution
    insert(all, self)
end)

function Item.All()
    return all
end

function Item:getEnergy()
    return self.energy
end

function Item:getEnergyDensity()
    return self.energy / self.mass
end

function Item:getDistribution()
    return self.distribution
end

function Item:getMass()
    return self.mass
end

function Item:getName()
    return self.name
end

function Item:setEnergy(energy)
    self.energy = energy
    return self
end

Item.Credit = Item("Credit")
Item.Credit.mass = 0

return Item
