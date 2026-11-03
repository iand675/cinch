// Mounts Agentation's toolbar. Imported only by FeedbackToolbar.astro in dev.
import { createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { Agentation, type AgentationProps } from 'agentation';

export function mountFeedbackToolbar(endpoint?: string) {
  const mount = document.createElement('div');
  mount.dataset.feedbackToolbarRoot = '';
  document.body.append(mount);
  const props: AgentationProps = { appName: 'Cinch website', endpoint };
  // Agentation's `props` parameter is optional, so inference would make P
  // include `undefined` and fail `P extends {}`; name P explicitly.
  createRoot(mount).render(createElement<AgentationProps>(Agentation, props));
}
