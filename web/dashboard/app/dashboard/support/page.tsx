'use client'

import { useState } from 'react'
import Link from 'next/link'
import { motion } from 'framer-motion'
import {
  HelpCircle, Plus, MessageSquare, Clock, CheckCircle, AlertCircle,
  ChevronRight, Search, FileText, Send, Paperclip,
  ExternalLink, Book, MessageCircle, Shield, CreditCard, Bug, Loader2
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import * as Tabs from '@radix-ui/react-tabs'
import { useSupportTickets, useCreateTicket, useFAQ } from '@/shared/hooks/useDashboard'
import { useCharacters } from '@/shared/hooks/useCharacters'
import type { SupportTicket } from '@/shared/api/endpoints'

const statusColors: Record<string, { bg: string; text: string }> = {
  open: { bg: 'bg-blue-500/20', text: 'text-blue-400' },
  pending: { bg: 'bg-amber-500/20', text: 'text-amber-400' },
  resolved: { bg: 'bg-emerald-500/20', text: 'text-emerald-400' },
  closed: { bg: 'bg-slate-500/20', text: 'text-slate-400' },
}

const priorityColors: Record<string, string> = {
  urgent: 'text-red-500',
  high: 'text-red-400',
  medium: 'text-amber-400',
  low: 'text-slate-400',
}

const categories: { id: SupportTicket['category']; name: string; icon: React.ElementType }[] = [
  { id: 'technical', name: 'Technical Issue', icon: Bug },
  { id: 'billing', name: 'Billing & Payments', icon: CreditCard },
  { id: 'account', name: 'Account Help', icon: Shield },
  { id: 'report', name: 'Report Player', icon: AlertCircle },
  { id: 'other', name: 'Other', icon: HelpCircle },
]

export default function SupportPage() {
  const [showNewTicket, setShowNewTicket] = useState(false)
  const [selectedCategory, setSelectedCategory] = useState<SupportTicket['category'] | ''>('')
  const [searchQuery, setSearchQuery] = useState('')
  const [ticketForm, setTicketForm] = useState({ subject: '', message: '' })
  
  // Real API hooks
  const { data: ticketsData, isLoading: ticketsLoading } = useSupportTickets()
  const { data: faqData, isLoading: faqLoading } = useFAQ()
  const { data: characters = [] } = useCharacters()
  const createTicket = useCreateTicket()

  const tickets = ticketsData?.data || []
  const faqCategories = faqData || []
  
  // Stats from real data
  const stats = {
    open: tickets.filter(t => t.status === 'open').length,
    pending: tickets.filter(t => t.status === 'pending').length,
    resolved: tickets.filter(t => t.status === 'resolved').length,
  }

  const filteredTickets = tickets.filter(ticket =>
    ticket.subject.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const handleSubmitTicket = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!selectedCategory || !ticketForm.subject || !ticketForm.message) return
    
    await createTicket.mutateAsync({
      subject: ticketForm.subject,
      category: selectedCategory,
      message: ticketForm.message,
    })
    setShowNewTicket(false)
    setTicketForm({ subject: '', message: '' })
    setSelectedCategory('')
  }

  const formatTimeAgo = (date: string) => {
    const d = new Date(date)
    const now = new Date()
    const diffMs = now.getTime() - d.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    const diffHours = Math.floor(diffMins / 60)
    const diffDays = Math.floor(diffHours / 24)
    
    if (diffMins < 60) return `${diffMins}m ago`
    if (diffHours < 24) return `${diffHours}h ago`
    return `${diffDays}d ago`
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-between"
      >
        <div>
          <h1 className="text-2xl font-bold text-white mb-1">Support Center</h1>
          <p className="text-slate-400">Get help or submit a support ticket</p>
        </div>
        <button
          onClick={() => setShowNewTicket(true)}
          className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition"
        >
          <Plus className="w-4 h-4" />
          New Ticket
        </button>
      </motion.div>

      {/* Quick Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <p className="text-2xl font-bold text-blue-400">{stats.open}</p>
          <p className="text-xs text-slate-500">Open Tickets</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <p className="text-2xl font-bold text-amber-400">{stats.pending}</p>
          <p className="text-xs text-slate-500">Awaiting Response</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <p className="text-2xl font-bold text-emerald-400">{stats.resolved}</p>
          <p className="text-xs text-slate-500">Resolved</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <p className="text-2xl font-bold text-purple-400">{tickets.length}</p>
          <p className="text-xs text-slate-500">Total Tickets</p>
        </div>
      </motion.div>

      {/* Main Content */}
      <Tabs.Root defaultValue="tickets">
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          <Tabs.Trigger
            value="tickets"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400 data-[state=inactive]:hover:text-white"
          >
            <MessageSquare className="w-4 h-4" />
            My Tickets
          </Tabs.Trigger>
          <Tabs.Trigger
            value="faq"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400 data-[state=inactive]:hover:text-white"
          >
            <Book className="w-4 h-4" />
            FAQ / Knowledge Base
          </Tabs.Trigger>
        </Tabs.List>

        {/* Tickets Tab */}
        <Tabs.Content value="tickets">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-4"
          >
            {/* Search */}
            <div className="relative">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
              <input
                type="text"
                placeholder="Search tickets..."
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
                className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
              />
            </div>

            {/* Tickets List */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
              {ticketsLoading ? (
                <div className="flex items-center justify-center py-12">
                  <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
                </div>
              ) : filteredTickets.length === 0 ? (
                <div className="text-center py-16">
                  <MessageSquare className="w-16 h-16 text-slate-700 mx-auto mb-4" />
                  <h3 className="text-xl font-semibold text-white mb-2">No tickets found</h3>
                  <p className="text-slate-500">Create a new ticket to get help</p>
                </div>
              ) : (
                <div className="divide-y divide-slate-700/50">
                  {filteredTickets.map((ticket, idx) => {
                    const statusColor = statusColors[ticket.status] || statusColors.open
                    return (
                      <motion.div
                        key={ticket.id}
                        initial={{ opacity: 0, x: -10 }}
                        animate={{ opacity: 1, x: 0 }}
                        transition={{ delay: idx * 0.05 }}
                      >
                        <Link
                          href={`/dashboard/support/${ticket.id}`}
                          className="flex items-center justify-between px-5 py-4 hover:bg-slate-700/30 transition group"
                        >
                          <div className="flex items-center gap-4 flex-1">
                            <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${statusColor.bg}`}>
                              {ticket.status === 'open' ? <MessageCircle className={`w-5 h-5 ${statusColor.text}`} /> :
                               ticket.status === 'pending' ? <Clock className={`w-5 h-5 ${statusColor.text}`} /> :
                               ticket.status === 'resolved' ? <CheckCircle className={`w-5 h-5 ${statusColor.text}`} /> :
                               <AlertCircle className={`w-5 h-5 ${statusColor.text}`} />}
                            </div>
                            <div className="flex-1 min-w-0">
                              <div className="flex items-center gap-2 mb-1">
                                <span className="text-slate-500 text-xs font-mono">{ticket.id}</span>
                                <span className={`px-2 py-0.5 rounded text-xs ${statusColor.bg} ${statusColor.text}`}>
                                  {ticket.status}
                                </span>
                                <span className={`text-xs ${priorityColors[ticket.priority]}`}>
                                  {ticket.priority}
                                </span>
                              </div>
                              <h3 className="text-white font-medium group-hover:text-amber-400 transition truncate">
                                {ticket.subject}
                              </h3>
                              <p className="text-slate-500 text-sm">
                                {ticket.category} • Updated {formatTimeAgo(ticket.updatedAt)}
                              </p>
                            </div>
                          </div>
                          <div className="flex items-center gap-4">
                            <div className="flex items-center gap-1 text-slate-500 text-sm">
                              <MessageSquare className="w-4 h-4" />
                              {ticket.messages.length}
                            </div>
                            <ChevronRight className="w-5 h-5 text-slate-600 group-hover:text-amber-400 transition" />
                          </div>
                        </Link>
                      </motion.div>
                    )
                  })}
                </div>
              )}
            </div>
          </motion.div>
        </Tabs.Content>

        {/* FAQ Tab */}
        <Tabs.Content value="faq">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-6"
          >
            {/* FAQ Categories */}
            {faqLoading ? (
              <div className="flex items-center justify-center py-12">
                <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
              </div>
            ) : (
              <div className="grid md:grid-cols-2 gap-6">
                {faqCategories.map((category, idx) => (
                  <motion.div
                    key={category.category}
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: idx * 0.05 }}
                    className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
                  >
                    <div className="flex items-center gap-3 mb-4">
                      <div className="w-10 h-10 bg-amber-500/20 rounded-lg flex items-center justify-center">
                        <Book className="w-5 h-5 text-amber-400" />
                      </div>
                      <h3 className="text-white font-medium">{category.category}</h3>
                    </div>
                    <ul className="space-y-2">
                      {category.items.map((item, itemIdx) => (
                        <li key={itemIdx}>
                          <Link
                            href="#"
                            className="flex items-center justify-between text-slate-300 hover:text-amber-400 transition text-sm py-2 border-b border-slate-700/50 last:border-0"
                          >
                            <span>{item.question}</span>
                            <ChevronRight className="w-4 h-4 text-slate-600" />
                          </Link>
                        </li>
                      ))}
                    </ul>
                  </motion.div>
                ))}
              </div>
            )}

            {/* Contact Info */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h3 className="text-white font-medium mb-4">Still need help?</h3>
              <div className="grid md:grid-cols-3 gap-4">
                <a
                  href="https://discord.gg/shadowot"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-3 p-4 bg-slate-900/50 rounded-lg hover:bg-slate-800/50 transition group"
                >
                  <div className="w-10 h-10 bg-indigo-500/20 rounded-lg flex items-center justify-center">
                    <MessageCircle className="w-5 h-5 text-indigo-400" />
                  </div>
                  <div className="flex-1">
                    <p className="text-white font-medium group-hover:text-amber-400 transition">Discord</p>
                    <p className="text-slate-500 text-sm">Join our community</p>
                  </div>
                  <ExternalLink className="w-4 h-4 text-slate-600" />
                </a>
                <a
                  href="mailto:support@shadowot.com"
                  className="flex items-center gap-3 p-4 bg-slate-900/50 rounded-lg hover:bg-slate-800/50 transition group"
                >
                  <div className="w-10 h-10 bg-emerald-500/20 rounded-lg flex items-center justify-center">
                    <Send className="w-5 h-5 text-emerald-400" />
                  </div>
                  <div className="flex-1">
                    <p className="text-white font-medium group-hover:text-amber-400 transition">Email</p>
                    <p className="text-slate-500 text-sm">support@shadowot.com</p>
                  </div>
                </a>
                <button
                  onClick={() => setShowNewTicket(true)}
                  className="flex items-center gap-3 p-4 bg-slate-900/50 rounded-lg hover:bg-slate-800/50 transition group"
                >
                  <div className="w-10 h-10 bg-amber-500/20 rounded-lg flex items-center justify-center">
                    <FileText className="w-5 h-5 text-amber-400" />
                  </div>
                  <div className="flex-1 text-left">
                    <p className="text-white font-medium group-hover:text-amber-400 transition">Submit Ticket</p>
                    <p className="text-slate-500 text-sm">Get personalized help</p>
                  </div>
                </button>
              </div>
            </div>
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>

      {/* New Ticket Dialog */}
      <Dialog.Root open={showNewTicket} onOpenChange={setShowNewTicket}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-lg max-h-[90vh] overflow-y-auto z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-2">Create Support Ticket</Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              Describe your issue and we'll get back to you as soon as possible.
            </Dialog.Description>

            <form onSubmit={handleSubmitTicket} className="space-y-4">
              {/* Category */}
              <div>
                <label className="text-slate-300 text-sm mb-2 block">Category</label>
                <div className="grid grid-cols-2 gap-2">
                  {categories.map(cat => (
                    <button
                      key={cat.id}
                      type="button"
                      onClick={() => setSelectedCategory(cat.id)}
                      className={`flex items-center gap-2 p-3 rounded-lg border transition text-left ${
                        selectedCategory === cat.id
                          ? 'border-amber-500 bg-amber-500/10'
                          : 'border-slate-700 hover:border-slate-600'
                      }`}
                    >
                      <cat.icon className={`w-4 h-4 ${selectedCategory === cat.id ? 'text-amber-400' : 'text-slate-400'}`} />
                      <span className={`text-sm ${selectedCategory === cat.id ? 'text-white' : 'text-slate-300'}`}>
                        {cat.name}
                      </span>
                    </button>
                  ))}
                </div>
              </div>

              {/* Subject */}
              <div>
                <label className="text-slate-300 text-sm mb-2 block">Subject</label>
                <input
                  type="text"
                  value={ticketForm.subject}
                  onChange={e => setTicketForm(f => ({ ...f, subject: e.target.value }))}
                  placeholder="Brief description of your issue"
                  className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                />
              </div>

              {/* Description */}
              <div>
                <label className="text-slate-300 text-sm mb-2 block">Description</label>
                <textarea
                  rows={5}
                  value={ticketForm.message}
                  onChange={e => setTicketForm(f => ({ ...f, message: e.target.value }))}
                  placeholder="Please provide as much detail as possible..."
                  className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-3 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50 resize-none"
                />
              </div>

              {/* Character (optional) */}
              <div>
                <label className="text-slate-300 text-sm mb-2 block">Related Character (optional)</label>
                <select className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50">
                  <option value="">Select a character</option>
                  {characters.map(char => (
                    <option key={char.id} value={char.id}>{char.name} ({char.level} {char.vocation})</option>
                  ))}
                </select>
              </div>

              {/* Attachments */}
              <div>
                <label className="text-slate-300 text-sm mb-2 block">Attachments</label>
                <div className="border-2 border-dashed border-slate-700 rounded-lg p-6 text-center hover:border-slate-600 transition cursor-pointer">
                  <Paperclip className="w-8 h-8 text-slate-500 mx-auto mb-2" />
                  <p className="text-slate-400 text-sm">Click to upload or drag and drop</p>
                  <p className="text-slate-500 text-xs mt-1">PNG, JPG, PDF up to 10MB</p>
                </div>
              </div>

              {/* Buttons */}
              <div className="flex gap-3 pt-4">
                <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                  Cancel
                </Dialog.Close>
                <button
                  type="submit"
                  disabled={!selectedCategory || !ticketForm.subject || !ticketForm.message || createTicket.isPending}
                  className="flex-1 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition disabled:opacity-50 flex items-center justify-center gap-2"
                >
                  {createTicket.isPending ? (
                    <>
                      <Loader2 className="w-4 h-4 animate-spin" />
                      Submitting...
                    </>
                  ) : (
                    'Submit Ticket'
                  )}
                </button>
              </div>
            </form>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
