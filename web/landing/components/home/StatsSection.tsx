'use client'

import { motion } from 'framer-motion'
import { TrendingUp, Users, Clock, Server, Sword, Trophy } from 'lucide-react'

const stats = [
  { label: 'Players Online', value: '5,247', icon: Users, change: '+12%' },
  { label: 'Accounts Created', value: '142,893', icon: TrendingUp, change: '+8%' },
  { label: 'Total Playtime', value: '2.4M hrs', icon: Clock, change: '+15%' },
  { label: 'Active Realms', value: '6', icon: Server, change: '' },
  { label: 'PvP Battles Today', value: '8,412', icon: Sword, change: '+23%' },
  { label: 'Achievements Unlocked', value: '892K', icon: Trophy, change: '+5%' },
]

export function StatsSection() {
  return (
    <section className="py-16 bg-shadow-900 border-y border-shadow-800">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-8">
          {stats.map((stat, index) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
              className="text-center"
            >
              <div className="flex items-center justify-center mb-3">
                <stat.icon className="w-5 h-5 text-accent-500 mr-2" />
                <span className="text-2xl lg:text-3xl font-bold text-white">{stat.value}</span>
              </div>
              <div className="flex items-center justify-center space-x-2">
                <span className="text-shadow-400 text-sm">{stat.label}</span>
                {stat.change && (
                  <span className="text-green-500 text-xs font-medium">{stat.change}</span>
                )}
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  )
}
