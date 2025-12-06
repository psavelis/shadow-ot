-- Shadow OT Skills Module
-- Displays player skills and stats

Skills = {}

local skillsWindow = nil
local player = nil

local skillNames = {
    [0] = 'Fist Fighting',
    [1] = 'Club Fighting',
    [2] = 'Sword Fighting',
    [3] = 'Axe Fighting',
    [4] = 'Distance Fighting',
    [5] = 'Shielding',
    [6] = 'Fishing'
}

function Skills.init()
    connect(g_game, {
        onGameStart = Skills.onGameStart,
        onGameEnd = Skills.onGameEnd
    })

    connect(LocalPlayer, {
        onStatsChange = Skills.onStatsChange,
        onSkillChange = Skills.onSkillChange
    })

    skillsWindow = g_ui.displayUI('skills')
    skillsWindow:hide()
end

function Skills.terminate()
    disconnect(g_game, {
        onGameStart = Skills.onGameStart,
        onGameEnd = Skills.onGameEnd
    })

    disconnect(LocalPlayer, {
        onStatsChange = Skills.onStatsChange,
        onSkillChange = Skills.onSkillChange
    })

    if skillsWindow then
        skillsWindow:destroy()
        skillsWindow = nil
    end
end

function Skills.onGameStart()
    player = g_game.getLocalPlayer()
    skillsWindow:show()
    Skills.refresh()
end

function Skills.onGameEnd()
    player = nil
    skillsWindow:hide()
end

function Skills.onStatsChange()
    Skills.updateStats()
end

function Skills.onSkillChange(skill, level, percent)
    Skills.updateSkill(skill)
end

function Skills.refresh()
    Skills.updateStats()
    for skillId = 0, 6 do
        Skills.updateSkill(skillId)
    end
end

function Skills.updateStats()
    if not player then return end

    -- Experience
    local expWidget = skillsWindow:getChildById('experience')
    expWidget:getChildById('value'):setText(formatNumber(player:getExperience()))

    -- Level
    local levelWidget = skillsWindow:getChildById('level')
    levelWidget:getChildById('value'):setText(tostring(player:getLevel()))
    levelWidget:getChildById('progress'):setPercent(player:getLevelPercent())

    -- Health
    local healthWidget = skillsWindow:getChildById('health')
    healthWidget:getChildById('value'):setText(string.format('%d / %d',
        player:getHealth(), player:getMaxHealth()))

    -- Mana
    local manaWidget = skillsWindow:getChildById('mana')
    manaWidget:getChildById('value'):setText(string.format('%d / %d',
        player:getMana(), player:getMaxMana()))

    -- Soul
    local soulWidget = skillsWindow:getChildById('soul')
    soulWidget:getChildById('value'):setText(tostring(player:getSoul()))

    -- Capacity
    local capWidget = skillsWindow:getChildById('capacity')
    capWidget:getChildById('value'):setText(string.format('%.2f', player:getFreeCapacity() / 100))

    -- Magic Level
    local mlWidget = skillsWindow:getChildById('magicLevel')
    local ml = player:getMagicLevel()
    local baseML = player:getBaseMagicLevel()
    if ml ~= baseML then
        mlWidget:getChildById('value'):setText(string.format('%d (%d)', ml, baseML))
        mlWidget:getChildById('value'):setColor('#00ff00')
    else
        mlWidget:getChildById('value'):setText(tostring(ml))
        mlWidget:getChildById('value'):setColor('#ffffff')
    end
    mlWidget:getChildById('progress'):setPercent(player:getMagicLevelPercent())

    -- Stamina
    local staminaWidget = skillsWindow:getChildById('stamina')
    local stamina = player:getStamina()
    local hours = math.floor(stamina / 60)
    local minutes = stamina % 60
    staminaWidget:getChildById('value'):setText(string.format('%02d:%02d', hours, minutes))

    if stamina > 2400 then
        staminaWidget:getChildById('value'):setColor('#00ff00')
    elseif stamina > 840 then
        staminaWidget:getChildById('value'):setColor('#ffffff')
    else
        staminaWidget:getChildById('value'):setColor('#ff0000')
    end
end

function Skills.updateSkill(skillId)
    if not player then return end

    local skillName = skillNames[skillId]
    if not skillName then return end

    local skillWidget = skillsWindow:getChildById('skill' .. skillId)
    if not skillWidget then return end

    local level = player:getSkillLevel(skillId)
    local baseLevel = player:getSkillBaseLevel(skillId)
    local percent = player:getSkillLevelPercent(skillId)

    if level ~= baseLevel then
        skillWidget:getChildById('value'):setText(string.format('%d (%d)', level, baseLevel))
        if level > baseLevel then
            skillWidget:getChildById('value'):setColor('#00ff00')
        else
            skillWidget:getChildById('value'):setColor('#ff0000')
        end
    else
        skillWidget:getChildById('value'):setText(tostring(level))
        skillWidget:getChildById('value'):setColor('#ffffff')
    end

    skillWidget:getChildById('progress'):setPercent(percent)
end

function formatNumber(num)
    local formatted = tostring(num)
    local k
    while true do
        formatted, k = string.gsub(formatted, '^(-?%d+)(%d%d%d)', '%1,%2')
        if k == 0 then break end
    end
    return formatted
end

function Skills.toggle()
    if skillsWindow:isVisible() then
        skillsWindow:hide()
    else
        skillsWindow:show()
        skillsWindow:raise()
    end
end

function Skills.show()
    skillsWindow:show()
    skillsWindow:raise()
end

function Skills.hide()
    skillsWindow:hide()
end
