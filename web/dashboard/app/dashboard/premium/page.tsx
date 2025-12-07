'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  Crown, Check, Star, Zap, Shield, Gift, Clock, CreditCard,
  ChevronRight, Sparkles, Rocket, TrendingUp, Award, Package
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'

const plans = [
  {
    id: 'monthly',
    name: 'Monthly',
    price: 9.99,
    period: 'month',
    coins: 250,
    popular: false,
    savings: null,
  },
  {
    id: 'quarterly',
    name: 'Quarterly',
    price: 24.99,
    period: '3 months',
    coins: 850,
    popular: true,
    savings: '17%',
  },
  {
    id: 'yearly',
    name: 'Yearly',
    price: 89.99,
    period: 'year',
    coins: 4000,
    popular: false,
    savings: '25%',
  },
]

const benefits = [
  { icon: Zap, title: 'Double Experience', desc: 'Earn 2x experience in all hunting zones' },
  { icon: Shield, title: 'Priority Access', desc: 'Skip login queues during peak hours' },
  { icon: Package, title: 'Extra Storage', desc: '+500 depot slots per character' },
  { icon: Star, title: 'Exclusive Outfits', desc: 'Access to premium-only cosmetics' },
  { icon: Award, title: 'VIP Support', desc: 'Priority customer support response' },
  { icon: Gift, title: 'Monthly Rewards', desc: 'Exclusive items delivered each month' },
  { icon: TrendingUp, title: 'Market Benefits', desc: 'Lower fees on marketplace trades' },
  { icon: Rocket, title: 'Early Access', desc: 'Try new features before everyone else' },
]

const transactionHistory = [
  { id: 1, type: 'subscription', desc: 'Quarterly Premium', amount: '$24.99', date: '2024-12-01', status: 'completed' },
  { id: 2, type: 'coins', desc: '1000 Shadow Coins', amount: '$9.99', date: '2024-11-15', status: 'completed' },
  { id: 3, type: 'subscription', desc: 'Monthly Premium', amount: '$9.99', date: '2024-11-01', status: 'completed' },
  { id: 4, type: 'coins', desc: '500 Shadow Coins', amount: '$4.99', date: '2024-10-20', status: 'completed' },
]

const coinPackages = [
  { id: 1, coins: 250, price: 2.49, bonus: 0 },
  { id: 2, coins: 500, price: 4.99, bonus: 0 },
  { id: 3, coins: 1000, price: 9.99, bonus: 50 },
  { id: 4, coins: 2500, price: 22.99, bonus: 250 },
  { id: 5, coins: 5000, price: 44.99, bonus: 750 },
  { id: 6, coins: 10000, price: 84.99, bonus: 2000 },
]

export default function PremiumPage() {
  const [selectedPlan, setSelectedPlan] = useState('quarterly')
  const [showPurchaseDialog, setShowPurchaseDialog] = useState(false)
  const [purchaseType, setPurchaseType] = useState<'premium' | 'coins'>('premium')

  const currentStatus = {
    active: true,
    plan: 'Quarterly',
    expiresAt: '2025-03-01',
    daysRemaining: 89,
    coins: 1547,
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h1 className="text-2xl font-bold text-white mb-1">Premium & Shop</h1>
        <p className="text-slate-400">Manage your subscription and purchase coins</p>
      </motion.div>

      {/* Current Status */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="bg-gradient-to-r from-amber-500/10 via-orange-500/10 to-red-500/10 border border-amber-500/30 rounded-2xl p-6"
      >
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <div className="w-16 h-16 bg-gradient-to-br from-amber-500 to-orange-600 rounded-xl flex items-center justify-center">
              <Crown className="w-8 h-8 text-white" />
            </div>
            <div>
              <div className="flex items-center gap-2 mb-1">
                <h2 className="text-xl font-bold text-white">Premium Active</h2>
                <span className="px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded-full">
                  {currentStatus.plan}
                </span>
              </div>
              <p className="text-slate-400">
                {currentStatus.daysRemaining} days remaining • Expires {currentStatus.expiresAt}
              </p>
            </div>
          </div>
          <div className="text-right">
            <p className="text-amber-400 text-sm mb-1">Shadow Coins</p>
            <p className="text-3xl font-bold text-white">{currentStatus.coins.toLocaleString()}</p>
          </div>
        </div>

        {/* Progress Bar */}
        <div className="mt-6">
          <div className="flex justify-between text-sm mb-2">
            <span className="text-slate-400">Subscription Progress</span>
            <span className="text-white">{currentStatus.daysRemaining}/90 days</span>
          </div>
          <div className="h-2 bg-slate-700/50 rounded-full overflow-hidden">
            <div
              className="h-full bg-gradient-to-r from-amber-500 to-orange-500 rounded-full"
              style={{ width: `${(currentStatus.daysRemaining / 90) * 100}%` }}
            />
          </div>
        </div>
      </motion.div>

      {/* Premium Benefits */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
      >
        <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <Sparkles className="w-5 h-5 text-amber-400" />
          Premium Benefits
        </h3>
        <div className="grid md:grid-cols-4 gap-4">
          {benefits.map((benefit, idx) => (
            <div key={idx} className="flex items-start gap-3 p-3 bg-slate-900/50 rounded-lg">
              <div className="w-8 h-8 bg-amber-500/20 rounded-lg flex items-center justify-center flex-shrink-0">
                <benefit.icon className="w-4 h-4 text-amber-400" />
              </div>
              <div>
                <p className="text-white font-medium text-sm">{benefit.title}</p>
                <p className="text-slate-500 text-xs">{benefit.desc}</p>
              </div>
            </div>
          ))}
        </div>
      </motion.div>

      {/* Subscription Plans */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3 }}
      >
        <h3 className="text-lg font-bold text-white mb-4">Extend Subscription</h3>
        <div className="grid md:grid-cols-3 gap-4">
          {plans.map(plan => (
            <button
              key={plan.id}
              onClick={() => {
                setSelectedPlan(plan.id)
                setPurchaseType('premium')
                setShowPurchaseDialog(true)
              }}
              className={`relative bg-slate-800/50 border rounded-xl p-6 text-left transition hover:border-amber-500/50 ${
                plan.popular ? 'border-amber-500/50' : 'border-slate-700/50'
              }`}
            >
              {plan.popular && (
                <span className="absolute -top-3 left-1/2 -translate-x-1/2 px-3 py-1 bg-gradient-to-r from-amber-500 to-orange-600 text-white text-xs font-medium rounded-full">
                  Most Popular
                </span>
              )}
              {plan.savings && (
                <span className="absolute top-4 right-4 px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded-full">
                  Save {plan.savings}
                </span>
              )}
              <h4 className="text-white font-bold text-lg mb-1">{plan.name}</h4>
              <div className="mb-4">
                <span className="text-3xl font-bold text-white">${plan.price}</span>
                <span className="text-slate-500 text-sm">/{plan.period}</span>
              </div>
              <div className="flex items-center gap-2 text-amber-400 text-sm">
                <Star className="w-4 h-4" />
                <span>+{plan.coins} Shadow Coins included</span>
              </div>
            </button>
          ))}
        </div>
      </motion.div>

      {/* Coin Packages */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.4 }}
      >
        <h3 className="text-lg font-bold text-white mb-4">Buy Shadow Coins</h3>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3">
          {coinPackages.map(pkg => (
            <button
              key={pkg.id}
              onClick={() => {
                setPurchaseType('coins')
                setShowPurchaseDialog(true)
              }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 text-center hover:border-amber-500/50 transition group"
            >
              <div className="w-10 h-10 bg-amber-500/20 rounded-full flex items-center justify-center mx-auto mb-2">
                <Star className="w-5 h-5 text-amber-400" />
              </div>
              <p className="text-white font-bold">{pkg.coins.toLocaleString()}</p>
              {pkg.bonus > 0 && (
                <p className="text-emerald-400 text-xs">+{pkg.bonus} bonus</p>
              )}
              <p className="text-slate-400 text-sm mt-1">${pkg.price}</p>
            </button>
          ))}
        </div>
      </motion.div>

      {/* Transaction History */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.5 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
      >
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-bold text-white flex items-center gap-2">
            <CreditCard className="w-5 h-5 text-slate-400" />
            Transaction History
          </h3>
          <button className="text-amber-400 hover:text-amber-300 text-sm flex items-center gap-1">
            View All
            <ChevronRight className="w-4 h-4" />
          </button>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-slate-700">
                <th className="text-left py-3 px-2 text-slate-500 font-medium text-sm">Type</th>
                <th className="text-left py-3 px-2 text-slate-500 font-medium text-sm">Description</th>
                <th className="text-left py-3 px-2 text-slate-500 font-medium text-sm">Amount</th>
                <th className="text-left py-3 px-2 text-slate-500 font-medium text-sm">Date</th>
                <th className="text-right py-3 px-2 text-slate-500 font-medium text-sm">Status</th>
              </tr>
            </thead>
            <tbody>
              {transactionHistory.map(tx => (
                <tr key={tx.id} className="border-b border-slate-700/50 last:border-0">
                  <td className="py-3 px-2">
                    <span className={`px-2 py-0.5 rounded text-xs ${
                      tx.type === 'subscription' ? 'bg-purple-500/20 text-purple-400' : 'bg-amber-500/20 text-amber-400'
                    }`}>
                      {tx.type}
                    </span>
                  </td>
                  <td className="py-3 px-2 text-white">{tx.desc}</td>
                  <td className="py-3 px-2 text-white font-medium">{tx.amount}</td>
                  <td className="py-3 px-2 text-slate-400">{tx.date}</td>
                  <td className="py-3 px-2 text-right">
                    <span className="text-emerald-400 text-sm">{tx.status}</span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </motion.div>

      {/* Purchase Dialog */}
      <Dialog.Root open={showPurchaseDialog} onOpenChange={setShowPurchaseDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md">
            <Dialog.Title className="text-xl font-bold text-white mb-2">
              {purchaseType === 'premium' ? 'Extend Premium' : 'Purchase Coins'}
            </Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              Select your payment method to complete the purchase.
            </Dialog.Description>

            <div className="space-y-4 mb-6">
              {/* Payment Methods */}
              {[
                { id: 'card', name: 'Credit/Debit Card', icon: CreditCard },
                { id: 'paypal', name: 'PayPal', icon: CreditCard },
                { id: 'crypto', name: 'Cryptocurrency', icon: Star },
              ].map(method => (
                <button
                  key={method.id}
                  className="w-full flex items-center gap-4 p-4 bg-slate-900/50 border border-slate-700 rounded-lg hover:border-amber-500/50 transition"
                >
                  <div className="w-10 h-10 bg-slate-800 rounded-lg flex items-center justify-center">
                    <method.icon className="w-5 h-5 text-slate-400" />
                  </div>
                  <span className="text-white">{method.name}</span>
                  <ChevronRight className="w-4 h-4 text-slate-600 ml-auto" />
                </button>
              ))}
            </div>

            <div className="flex gap-3">
              <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                Cancel
              </Dialog.Close>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}


