'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Calculator, Sword, Zap, Coins, Target, TrendingUp,
  Shield, Heart, Clock, Award, ChevronDown, RotateCcw,
  Flame, Snowflake, Leaf, Moon, Sun, Sparkles, Info,
  Loader2
} from 'lucide-react'
import { useCreatures, getCreatureSprite } from '@shadow-ot/shared'

type CalculatorType = 'damage' | 'experience' | 'loot' | 'training' | 'imbue'

const vocations = ['Knight', 'Paladin', 'Sorcerer', 'Druid']
const skills = ['Sword', 'Axe', 'Club', 'Distance', 'Magic Level', 'Shielding', 'Fist']
const elements = [
  { name: 'Physical', icon: Sword, color: 'slate' },
  { name: 'Fire', icon: Flame, color: 'orange' },
  { name: 'Ice', icon: Snowflake, color: 'cyan' },
  { name: 'Energy', icon: Zap, color: 'purple' },
  { name: 'Earth', icon: Leaf, color: 'green' },
  { name: 'Holy', icon: Sun, color: 'yellow' },
  { name: 'Death', icon: Moon, color: 'slate' },
]

export default function ToolsPage() {
  const [activeCalculator, setActiveCalculator] = useState<CalculatorType>('damage')
  
  // Fetch creatures from API for quick reference
  const { data: creatures, isLoading: creaturesLoading } = useCreatures({ pageSize: 10 })
  
  // Damage Calculator State
  const [playerLevel, setPlayerLevel] = useState(200)
  const [skill, setSkill] = useState(100)
  const [weaponAttack, setWeaponAttack] = useState(50)
  const [targetArmor, setTargetArmor] = useState(30)
  const [critical, setCritical] = useState(false)
  const [elementalBonus, setElementalBonus] = useState(0)
  
  // Experience Calculator State
  const [currentLevel, setCurrentLevel] = useState(100)
  const [targetLevel, setTargetLevel] = useState(200)
  const [expPerHour, setExpPerHour] = useState(2000000)
  const [stamina, setStamina] = useState(42)
  const [bonusExp, setBonusExp] = useState(0)
  
  // Loot Calculator State
  const [huntDuration, setHuntDuration] = useState(1)
  const [killsPerHour, setKillsPerHour] = useState(150)
  const [avgLoot, setAvgLoot] = useState(5000)
  const [lootBonus, setLootBonus] = useState(0)
  const [supplies, setSupplies] = useState(50000)

  // Calculate damage
  const calculateDamage = () => {
    const baseDamage = (skill * 0.085 + playerLevel * 0.2) * (weaponAttack / 100) * 100
    const reduction = Math.max(0, targetArmor * 0.5)
    const finalDamage = baseDamage - reduction
    const critDamage = critical ? finalDamage * 1.5 : finalDamage
    const elementalDamage = critDamage * (1 + elementalBonus / 100)
    return {
      min: Math.floor(elementalDamage * 0.7),
      max: Math.floor(elementalDamage * 1.3),
      avg: Math.floor(elementalDamage),
      dps: Math.floor(elementalDamage * 2), // Assuming 2 attacks per second
    }
  }

  // Calculate experience needed
  const calculateExperience = () => {
    const expFormula = (level: number) => Math.floor((50 * level * level * level - 150 * level * level + 400 * level) / 3)
    const currentExp = expFormula(currentLevel)
    const targetExp = expFormula(targetLevel)
    const expNeeded = targetExp - currentExp
    const effectiveExpPerHour = expPerHour * (1 + bonusExp / 100) * (stamina > 39 ? 1.5 : stamina > 0 ? 1 : 0.5)
    const hoursNeeded = expNeeded / effectiveExpPerHour
    return {
      expNeeded,
      hoursNeeded: Math.ceil(hoursNeeded),
      daysNeeded: Math.ceil(hoursNeeded / 8), // Assuming 8h/day
      staminaHours: Math.ceil(hoursNeeded * (42 / stamina)),
    }
  }

  // Calculate loot profit
  const calculateLoot = () => {
    const totalKills = huntDuration * killsPerHour
    const grossLoot = totalKills * avgLoot * (1 + lootBonus / 100)
    const netProfit = grossLoot - (supplies * huntDuration)
    const profitPerHour = netProfit / huntDuration
    return {
      totalKills,
      grossLoot: Math.floor(grossLoot),
      netProfit: Math.floor(netProfit),
      profitPerHour: Math.floor(profitPerHour),
    }
  }

  const damage = calculateDamage()
  const experience = calculateExperience()
  const loot = calculateLoot()

  const calculators = [
    { id: 'damage', label: 'Damage', icon: Sword, color: 'red' },
    { id: 'experience', label: 'Experience', icon: Zap, color: 'purple' },
    { id: 'loot', label: 'Loot/Profit', icon: Coins, color: 'amber' },
    { id: 'training', label: 'Training', icon: Target, color: 'blue' },
    { id: 'imbue', label: 'Imbuements', icon: Sparkles, color: 'cyan' },
  ]

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      {/* Hero */}
      <section className="relative py-20 overflow-hidden">
        <div className="absolute inset-0 bg-[url('/grid.svg')] opacity-5" />
        <div className="absolute inset-0 bg-gradient-to-b from-blue-500/5 via-transparent to-transparent" />
        
        <div className="container mx-auto px-4 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center max-w-3xl mx-auto"
          >
            <div className="inline-flex items-center gap-2 px-4 py-2 bg-blue-500/10 border border-blue-500/30 rounded-full text-blue-400 text-sm mb-6">
              <Calculator className="w-4 h-4" />
              Game Tools & Calculators
            </div>
            <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
              Player <span className="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-cyan-500">Tools</span>
            </h1>
            <p className="text-slate-400 text-lg">
              Calculate damage, experience, loot profits, and plan your character progression
            </p>
          </motion.div>
        </div>
      </section>

      {/* Calculator Tabs */}
      <section className="container mx-auto px-4 -mt-8 mb-8">
        <div className="flex flex-wrap justify-center gap-2">
          {calculators.map((calc) => {
            const Icon = calc.icon
            const isActive = activeCalculator === calc.id
            return (
              <button
                key={calc.id}
                onClick={() => setActiveCalculator(calc.id as CalculatorType)}
                className={`flex items-center gap-2 px-5 py-3 rounded-xl border transition ${
                  isActive
                    ? `bg-${calc.color}-500/20 border-${calc.color}-500/50 text-${calc.color}-400`
                    : 'bg-slate-800/50 border-slate-700/50 text-slate-400 hover:border-slate-600'
                }`}
              >
                <Icon className="w-5 h-5" />
                <span className="font-medium">{calc.label}</span>
              </button>
            )
          })}
        </div>
      </section>

      {/* Calculator Content */}
      <section className="container mx-auto px-4 pb-20">
        <div className="max-w-4xl mx-auto">
          {/* Damage Calculator */}
          {activeCalculator === 'damage' && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-2xl overflow-hidden"
            >
              <div className="p-6 border-b border-slate-700">
                <h2 className="text-xl font-bold text-white flex items-center gap-2">
                  <Sword className="w-6 h-6 text-red-400" />
                  Damage Calculator
                </h2>
                <p className="text-slate-400 text-sm mt-1">Calculate your attack damage against enemies</p>
              </div>

              <div className="p-6">
                <div className="grid md:grid-cols-2 gap-6">
                  {/* Inputs */}
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Player Level</label>
                      <input
                        type="number"
                        value={playerLevel}
                        onChange={(e) => setPlayerLevel(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-red-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Skill Level</label>
                      <input
                        type="number"
                        value={skill}
                        onChange={(e) => setSkill(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-red-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Weapon Attack</label>
                      <input
                        type="number"
                        value={weaponAttack}
                        onChange={(e) => setWeaponAttack(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-red-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Target Armor</label>
                      <input
                        type="number"
                        value={targetArmor}
                        onChange={(e) => setTargetArmor(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-red-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Elemental Bonus %</label>
                      <input
                        type="number"
                        value={elementalBonus}
                        onChange={(e) => setElementalBonus(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-red-500/50"
                      />
                    </div>
                    <label className="flex items-center gap-3 cursor-pointer">
                      <input
                        type="checkbox"
                        checked={critical}
                        onChange={(e) => setCritical(e.target.checked)}
                        className="w-5 h-5 bg-slate-900 border-slate-700 rounded text-red-500"
                      />
                      <span className="text-slate-300">Critical Hit (1.5x damage)</span>
                    </label>
                  </div>

                  {/* Results */}
                  <div className="bg-gradient-to-br from-red-500/10 to-orange-500/5 border border-red-500/30 rounded-xl p-6">
                    <h3 className="text-white font-bold mb-4 flex items-center gap-2">
                      <Target className="w-5 h-5 text-red-400" />
                      Damage Output
                    </h3>
                    <div className="space-y-4">
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Minimum</span>
                        <span className="text-red-400 font-bold text-xl">{damage.min.toLocaleString()}</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Maximum</span>
                        <span className="text-red-400 font-bold text-xl">{damage.max.toLocaleString()}</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Average</span>
                        <span className="text-amber-400 font-bold text-xl">{damage.avg.toLocaleString()}</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-gradient-to-r from-red-500/20 to-orange-500/20 border border-red-500/30 rounded-lg">
                        <span className="text-white font-medium">DPS (2 hits/s)</span>
                        <span className="text-red-400 font-bold text-2xl">{damage.dps.toLocaleString()}</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </motion.div>
          )}

          {/* Experience Calculator */}
          {activeCalculator === 'experience' && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-2xl overflow-hidden"
            >
              <div className="p-6 border-b border-slate-700">
                <h2 className="text-xl font-bold text-white flex items-center gap-2">
                  <Zap className="w-6 h-6 text-purple-400" />
                  Experience Calculator
                </h2>
                <p className="text-slate-400 text-sm mt-1">Plan your leveling journey</p>
              </div>

              <div className="p-6">
                <div className="grid md:grid-cols-2 gap-6">
                  {/* Inputs */}
                  <div className="space-y-4">
                    <div className="grid grid-cols-2 gap-4">
                      <div>
                        <label className="block text-sm text-slate-400 mb-2">Current Level</label>
                        <input
                          type="number"
                          value={currentLevel}
                          onChange={(e) => setCurrentLevel(Number(e.target.value))}
                          className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-purple-500/50"
                        />
                      </div>
                      <div>
                        <label className="block text-sm text-slate-400 mb-2">Target Level</label>
                        <input
                          type="number"
                          value={targetLevel}
                          onChange={(e) => setTargetLevel(Number(e.target.value))}
                          className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-purple-500/50"
                        />
                      </div>
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Base Exp/Hour</label>
                      <input
                        type="number"
                        value={expPerHour}
                        onChange={(e) => setExpPerHour(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-purple-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Stamina (hours)</label>
                      <input
                        type="number"
                        value={stamina}
                        onChange={(e) => setStamina(Number(e.target.value))}
                        max={42}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-purple-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Bonus XP % (prey, events, etc.)</label>
                      <input
                        type="number"
                        value={bonusExp}
                        onChange={(e) => setBonusExp(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-purple-500/50"
                      />
                    </div>
                  </div>

                  {/* Results */}
                  <div className="bg-gradient-to-br from-purple-500/10 to-violet-500/5 border border-purple-500/30 rounded-xl p-6">
                    <h3 className="text-white font-bold mb-4 flex items-center gap-2">
                      <TrendingUp className="w-5 h-5 text-purple-400" />
                      Leveling Progress
                    </h3>
                    <div className="space-y-4">
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Experience Needed</span>
                        <span className="text-purple-400 font-bold">{experience.expNeeded.toLocaleString()}</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Hours Needed</span>
                        <span className="text-purple-400 font-bold">{experience.hoursNeeded} hours</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Days (8h/day)</span>
                        <span className="text-amber-400 font-bold">{experience.daysNeeded} days</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-gradient-to-r from-purple-500/20 to-violet-500/20 border border-purple-500/30 rounded-lg">
                        <span className="text-white font-medium">With Full Stamina</span>
                        <span className="text-purple-400 font-bold text-xl">{experience.staminaHours}h</span>
                      </div>
                    </div>

                    <div className="mt-4 p-3 bg-blue-500/10 border border-blue-500/30 rounded-lg">
                      <p className="text-blue-400 text-sm flex items-center gap-2">
                        <Info className="w-4 h-4" />
                        Green stamina (first 42h) gives 50% bonus XP!
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </motion.div>
          )}

          {/* Loot Calculator */}
          {activeCalculator === 'loot' && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-2xl overflow-hidden"
            >
              <div className="p-6 border-b border-slate-700">
                <h2 className="text-xl font-bold text-white flex items-center gap-2">
                  <Coins className="w-6 h-6 text-amber-400" />
                  Loot & Profit Calculator
                </h2>
                <p className="text-slate-400 text-sm mt-1">Calculate your hunting profits</p>
              </div>

              <div className="p-6">
                <div className="grid md:grid-cols-2 gap-6">
                  {/* Inputs */}
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Hunt Duration (hours)</label>
                      <input
                        type="number"
                        value={huntDuration}
                        onChange={(e) => setHuntDuration(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-amber-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Kills Per Hour</label>
                      <input
                        type="number"
                        value={killsPerHour}
                        onChange={(e) => setKillsPerHour(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-amber-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Average Loot Per Kill (gold)</label>
                      <input
                        type="number"
                        value={avgLoot}
                        onChange={(e) => setAvgLoot(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-amber-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Loot Bonus % (prey, boost)</label>
                      <input
                        type="number"
                        value={lootBonus}
                        onChange={(e) => setLootBonus(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-amber-500/50"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-slate-400 mb-2">Supplies Per Hour (gold)</label>
                      <input
                        type="number"
                        value={supplies}
                        onChange={(e) => setSupplies(Number(e.target.value))}
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-amber-500/50"
                      />
                    </div>
                  </div>

                  {/* Results */}
                  <div className="bg-gradient-to-br from-amber-500/10 to-yellow-500/5 border border-amber-500/30 rounded-xl p-6">
                    <h3 className="text-white font-bold mb-4 flex items-center gap-2">
                      <Award className="w-5 h-5 text-amber-400" />
                      Hunt Summary
                    </h3>
                    <div className="space-y-4">
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Total Kills</span>
                        <span className="text-white font-bold">{loot.totalKills.toLocaleString()}</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Gross Loot</span>
                        <span className="text-amber-400 font-bold">{loot.grossLoot.toLocaleString()} gp</span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-slate-900/50 rounded-lg">
                        <span className="text-slate-400">Net Profit</span>
                        <span className={`font-bold ${loot.netProfit >= 0 ? 'text-emerald-400' : 'text-red-400'}`}>
                          {loot.netProfit.toLocaleString()} gp
                        </span>
                      </div>
                      <div className="flex justify-between items-center p-3 bg-gradient-to-r from-amber-500/20 to-yellow-500/20 border border-amber-500/30 rounded-lg">
                        <span className="text-white font-medium">Profit/Hour</span>
                        <span className={`font-bold text-xl ${loot.profitPerHour >= 0 ? 'text-emerald-400' : 'text-red-400'}`}>
                          {loot.profitPerHour.toLocaleString()} gp
                        </span>
                      </div>
                    </div>

                    <div className="mt-4 p-3 bg-emerald-500/10 border border-emerald-500/30 rounded-lg">
                      <p className="text-emerald-400 text-sm">
                        💡 Tip: Use prey on boosted creatures for +200% loot!
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            </motion.div>
          )}

          {/* Training Calculator Placeholder */}
          {activeCalculator === 'training' && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-2xl p-12 text-center"
            >
              <Target className="w-16 h-16 text-blue-400 mx-auto mb-4" />
              <h2 className="text-2xl font-bold text-white mb-2">Training Calculator</h2>
              <p className="text-slate-400">Calculate skill training time and offline training progress</p>
              <p className="text-blue-400 mt-4">Coming soon with more features!</p>
            </motion.div>
          )}

          {/* Imbuement Calculator Placeholder */}
          {activeCalculator === 'imbue' && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-2xl p-12 text-center"
            >
              <Sparkles className="w-16 h-16 text-cyan-400 mx-auto mb-4" />
              <h2 className="text-2xl font-bold text-white mb-2">Imbuement Calculator</h2>
              <p className="text-slate-400">Calculate imbuement costs and find the best value</p>
              <p className="text-cyan-400 mt-4">Coming soon with all imbuement types!</p>
            </motion.div>
          )}
        </div>
      </section>

      {/* Quick Reference Tables */}
      <section className="container mx-auto px-4 pb-20">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-xl font-bold text-white mb-4 flex items-center gap-2">
            <Info className="w-5 h-5 text-slate-400" />
            Monster Quick Reference
          </h2>
          <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
            {creaturesLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
              </div>
            ) : creatures?.data && creatures.data.length > 0 ? (
              <table className="w-full">
                <thead className="bg-slate-900/50">
                  <tr className="text-slate-400 text-sm">
                    <th className="text-left p-4">Monster</th>
                    <th className="text-right p-4">HP</th>
                    <th className="text-right p-4">Experience</th>
                    <th className="text-right p-4">Difficulty</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-700/50">
                  {creatures.data.map((creature) => (
                    <tr key={creature.id} className="hover:bg-slate-700/30 transition">
                      <td className="p-4">
                        <div className="flex items-center gap-3">
                          <img 
                            src={getCreatureSprite(creature.name)} 
                            alt={creature.name}
                            className="w-8 h-8 object-contain"
                            onError={(e) => {
                              (e.target as HTMLImageElement).src = '/icons/creatures/default.png'
                            }}
                          />
                          <span className="text-white font-medium">{creature.name}</span>
                        </div>
                      </td>
                      <td className="p-4 text-right text-red-400">{creature.health.toLocaleString()}</td>
                      <td className="p-4 text-right text-amber-400">{creature.experience.toLocaleString()}</td>
                      <td className="p-4 text-right">
                        <span className={`px-2 py-1 rounded text-xs ${
                          creature.difficulty === 'Hard' ? 'bg-red-500/20 text-red-400' :
                          creature.difficulty === 'Medium' ? 'bg-amber-500/20 text-amber-400' :
                          'bg-green-500/20 text-green-400'
                        }`}>
                          {creature.difficulty}
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <div className="text-center py-8 text-slate-400">
                No creature data available
              </div>
            )}
          </div>
        </div>
      </section>
    </main>
  )
}

