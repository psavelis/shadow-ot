'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { Shield, Users, Crown, Swords, MessageSquare, Settings, UserPlus, LogOut, Trophy, Globe } from 'lucide-react'

const guild = {
  name: 'Dark Legion',
  description: 'The oldest and most powerful guild in Shadowveil. We dominate PvP and control the best hunting grounds.',
  motd: 'Remember: Double XP event this weekend! Mandatory boss raids on Saturday at 20:00 server time.',
  realm: 'shadowveil',
  level: 45,
  members: 156,
  founded: '2022-03-15',
  leader: 'ShadowMaster',
  myRank: 'Vice Leader',
  wars: [
    { opponent: 'Phoenix Rising', kills: 45, deaths: 23, status: 'active' },
    { opponent: 'Storm Brigade', kills: 12, deaths: 8, status: 'ended' },
  ],
}

const members = [
  { id: '1', name: 'ShadowMaster', vocation: 'Elite Knight', level: 450, rank: 'Leader', online: true, joined: '2022-03-15' },
  { id: '2', name: 'MysticDruid', vocation: 'Elder Druid', level: 380, rank: 'Vice Leader', online: true, joined: '2022-03-20' },
  { id: '3', name: 'DragonSlayer', vocation: 'Royal Paladin', level: 412, rank: 'Vice Leader', online: false, joined: '2022-04-01' },
  { id: '4', name: 'StormMage', vocation: 'Master Sorcerer', level: 356, rank: 'Senior Member', online: true, joined: '2022-05-15' },
  { id: '5', name: 'IronKnight', vocation: 'Elite Knight', level: 298, rank: 'Member', online: false, joined: '2023-01-10' },
  { id: '6', name: 'HealerPro', vocation: 'Elder Druid', level: 267, rank: 'Member', online: true, joined: '2023-06-20' },
]

const ranks = [
  { name: 'Leader', members: 1, color: 'text-yellow-400' },
  { name: 'Vice Leader', members: 2, color: 'text-purple-400' },
  { name: 'Senior Member', members: 15, color: 'text-blue-400' },
  { name: 'Member', members: 98, color: 'text-green-400' },
  { name: 'Recruit', members: 40, color: 'text-shadow-400' },
]

const vocationColors: Record<string, string> = {
  'Elite Knight': 'text-red-400',
  'Royal Paladin': 'text-yellow-400',
  'Master Sorcerer': 'text-blue-400',
  'Elder Druid': 'text-green-400',
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

export default function GuildPage() {
  const [activeTab, setActiveTab] = useState<'overview' | 'members' | 'wars' | 'settings'>('overview')
  const [searchQuery, setSearchQuery] = useState('')

  const filteredMembers = members.filter(m => 
    m.name.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const hasGuild = true // Would be from user state

  if (!hasGuild) {
    return (
      <div className="max-w-2xl mx-auto text-center py-16">
        <Shield className="w-20 h-20 text-shadow-700 mx-auto mb-6" />
        <h1 className="text-2xl font-display font-bold mb-4">No Guild</h1>
        <p className="text-shadow-400 mb-8">
          You are not a member of any guild. Join an existing guild or create your own!
        </p>
        <div className="flex justify-center gap-4">
          <button className="btn-primary">Create Guild</button>
          <button className="btn-secondary">Browse Guilds</button>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center gap-4 mb-4">
          <div className="w-16 h-16 rounded-xl bg-shadow-700 flex items-center justify-center">
            <Shield className="w-8 h-8 text-accent-500" />
          </div>
          <div>
            <h1 className="text-2xl font-display font-bold">{guild.name}</h1>
            <p className="text-shadow-400">Level {guild.level} • {guild.members} members</p>
          </div>
        </div>
        <p className="text-shadow-300">{guild.description}</p>
      </motion.div>

      {/* MOTD */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }}>
        <div className="card bg-gradient-to-r from-accent-500/10 to-purple-500/10 border-accent-500/30">
          <div className="flex items-start gap-3">
            <MessageSquare className="w-5 h-5 text-accent-500 flex-shrink-0 mt-0.5" />
            <div>
              <p className="text-sm font-medium text-accent-400 mb-1">Message of the Day</p>
              <p className="text-white">{guild.motd}</p>
            </div>
          </div>
        </div>
      </motion.div>

      {/* Tabs */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 }}>
        <div className="flex gap-2 border-b border-shadow-700 pb-2">
          {(['overview', 'members', 'wars', 'settings'] as const).map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors capitalize ${
                activeTab === tab
                  ? 'bg-accent-500 text-white'
                  : 'text-shadow-400 hover:text-white hover:bg-shadow-800'
              }`}
            >
              {tab}
            </button>
          ))}
        </div>
      </motion.div>

      {/* Overview Tab */}
      {activeTab === 'overview' && (
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} className="grid lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2 space-y-6">
            {/* Stats */}
            <div className="grid sm:grid-cols-3 gap-4">
              <div className="stat-card">
                <Users className="w-5 h-5 text-accent-500 mb-2" />
                <h3 className="text-2xl font-bold text-white">{guild.members}</h3>
                <p className="text-shadow-400 text-sm">Members</p>
              </div>
              <div className="stat-card">
                <Trophy className="w-5 h-5 text-yellow-500 mb-2" />
                <h3 className="text-2xl font-bold text-white">{guild.level}</h3>
                <p className="text-shadow-400 text-sm">Guild Level</p>
              </div>
              <div className="stat-card">
                <Swords className="w-5 h-5 text-red-500 mb-2" />
                <h3 className="text-2xl font-bold text-white">{guild.wars.filter(w => w.status === 'active').length}</h3>
                <p className="text-shadow-400 text-sm">Active Wars</p>
              </div>
            </div>

            {/* Online Members */}
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
                <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                Online Now ({members.filter(m => m.online).length})
              </h2>
              <div className="grid sm:grid-cols-2 gap-3">
                {members.filter(m => m.online).map((member) => (
                  <div key={member.id} className="flex items-center gap-3 p-3 bg-shadow-800/50 rounded-lg">
                    <div className="w-10 h-10 rounded-lg bg-shadow-700 flex items-center justify-center">
                      <Shield className={`w-5 h-5 ${vocationColors[member.vocation]}`} />
                    </div>
                    <div>
                      <p className="text-white font-medium">{member.name}</p>
                      <p className="text-shadow-500 text-sm">Level {member.level} {member.vocation}</p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            {/* Your Status */}
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-4">Your Status</h2>
              <div className="space-y-3 text-sm">
                <div className="flex justify-between">
                  <span className="text-shadow-400">Rank</span>
                  <span className="text-purple-400 font-medium">{guild.myRank}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-shadow-400">Joined</span>
                  <span className="text-white">{formatDate('2022-03-20')}</span>
                </div>
              </div>
              <div className="mt-4 pt-4 border-t border-shadow-700">
                <button className="w-full btn-secondary text-sm flex items-center justify-center gap-2 text-red-400 hover:bg-red-500/20">
                  <LogOut className="w-4 h-4" />
                  Leave Guild
                </button>
              </div>
            </div>

            {/* Ranks */}
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-4">Ranks</h2>
              <div className="space-y-2">
                {ranks.map((rank) => (
                  <div key={rank.name} className="flex items-center justify-between py-2">
                    <span className={rank.color}>{rank.name}</span>
                    <span className="text-shadow-500 text-sm">{rank.members}</span>
                  </div>
                ))}
              </div>
            </div>

            {/* Info */}
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-4">Guild Info</h2>
              <div className="space-y-3 text-sm">
                <div className="flex items-center gap-2">
                  <Crown className="w-4 h-4 text-yellow-500" />
                  <span className="text-shadow-400">Leader:</span>
                  <span className="text-white">{guild.leader}</span>
                </div>
                <div className="flex items-center gap-2">
                  <Globe className="w-4 h-4 text-shadow-500" />
                  <span className="text-shadow-400">Realm:</span>
                  <span className="text-red-400 capitalize">{guild.realm}</span>
                </div>
                <div className="flex items-center gap-2">
                  <Shield className="w-4 h-4 text-shadow-500" />
                  <span className="text-shadow-400">Founded:</span>
                  <span className="text-white">{formatDate(guild.founded)}</span>
                </div>
              </div>
            </div>
          </div>
        </motion.div>
      )}

      {/* Members Tab */}
      {activeTab === 'members' && (
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} className="card">
          <div className="flex items-center justify-between mb-4">
            <div className="relative flex-1 max-w-md">
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search members..."
                className="w-full px-4 py-2 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50"
              />
            </div>
            <button className="btn-primary flex items-center gap-2">
              <UserPlus className="w-4 h-4" />
              Invite
            </button>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-shadow-700">
                  <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Name</th>
                  <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Vocation</th>
                  <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Level</th>
                  <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Rank</th>
                  <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Status</th>
                  <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Joined</th>
                </tr>
              </thead>
              <tbody>
                {filteredMembers.map((member) => (
                  <tr key={member.id} className="border-b border-shadow-800 last:border-0 hover:bg-shadow-800/50">
                    <td className="py-3 px-4 font-medium text-white">{member.name}</td>
                    <td className={`py-3 px-4 ${vocationColors[member.vocation]}`}>{member.vocation}</td>
                    <td className="py-3 px-4 text-white">{member.level}</td>
                    <td className="py-3 px-4">
                      <span className={ranks.find(r => r.name === member.rank)?.color}>
                        {member.rank}
                      </span>
                    </td>
                    <td className="py-3 px-4">
                      {member.online ? (
                        <span className="flex items-center gap-1 text-green-400">
                          <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                          Online
                        </span>
                      ) : (
                        <span className="text-shadow-500">Offline</span>
                      )}
                    </td>
                    <td className="py-3 px-4 text-shadow-500">{formatDate(member.joined)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </motion.div>
      )}

      {/* Wars Tab */}
      {activeTab === 'wars' && (
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} className="space-y-4">
          {guild.wars.map((war, index) => (
            <div key={index} className="card">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                  <Swords className={`w-8 h-8 ${war.status === 'active' ? 'text-red-500' : 'text-shadow-500'}`} />
                  <div>
                    <h3 className="text-lg font-semibold text-white">{guild.name} vs {war.opponent}</h3>
                    <span className={`text-sm ${war.status === 'active' ? 'text-red-400' : 'text-shadow-500'}`}>
                      {war.status === 'active' ? 'Active War' : 'War Ended'}
                    </span>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-2xl font-bold">
                    <span className="text-green-400">{war.kills}</span>
                    <span className="text-shadow-500 mx-2">-</span>
                    <span className="text-red-400">{war.deaths}</span>
                  </p>
                  <p className="text-shadow-500 text-sm">Kills - Deaths</p>
                </div>
              </div>
            </div>
          ))}
          <button className="btn-secondary w-full flex items-center justify-center gap-2">
            <Swords className="w-4 h-4" />
            Declare War
          </button>
        </motion.div>
      )}

      {/* Settings Tab */}
      {activeTab === 'settings' && (
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} className="card">
          <h2 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
            <Settings className="w-5 h-5 text-accent-500" />
            Guild Settings
          </h2>
          <p className="text-shadow-500">Only guild leaders can modify settings.</p>
        </motion.div>
      )}
    </div>
  )
}


