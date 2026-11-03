// Mounts Agentation's toolbar. Imported only by FeedbackToolbar.astro in dev.
import { createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { Agentation, type AgentationProps } from 'agentation';

export function mountFeedbackToolbar(endpoint?: string) {
  const mount = document.createElement('div');
  mount.dataset.feedbackToolbarRoot = '';
  document.body.append(mount);
  const props: AgentationProps = { appName: 'Cinch website', endpoint };
  createRoot(mount).render(createElement(Agentation, props));
}
